# Error recovery

By default, the parser will stop as soon as it encounters an error.
Sometimes though we would like to try and recover and keep going.
LALRPOP can support this, but you have to help it by defining various
"error recovery" points in your grammar. This is done by using a
special `!` token: this token only occurs when the parser
encounters an error in the input. When an error does occur, the parser
will try to recover and keep going; it does this by injecting the
`!` token into the stream, executing any actions that it can, and
then dropping input tokens until it finds something that lets it
continue.

Let's see how we can use error recovery to attempt to find multiple
errors during parsing. First we need a way to return multiple errors
as this is not something that LALRPOP does by itself so we add a `Vec`
storing the errors we found during parsing. Since the result of `!`
contains a token, error recovery requires that tokens can be cloned.

```
grammar<'err>(errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>);
```

Since an alternative containing `!` is expected to return the same type of
value as the other alternatives in the production we add an extra variant to
`Expr` to indicate that an error was found.

```rust
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Error,
}
```

Finally we modify the grammar, adding a third alternative containing `!`
which simply stores the `ErrorRecovery` value received from `!` in `errors` and
returns an `Expr::Error`. The value of the error token will be a [`ParseError`
value](https://docs.rs/lalrpop-util/0.12.1/lalrpop_util/enum.ParseError.html).

```lalrpop
Term: Box<Expr> = {
    Num => Box::new(Expr::Number(<>)),
    "(" <Expr> ")",
    ! => { errors.push(<>); Box::new(Expr::Error) },
};
```

Now we can add a test that includes various errors (e.g., missing
operands). You can see that the parser recovered from missing operands
by inserting this `!` token where necessary.

```rust
#[test]
fn calculator6() {
    let mut errors = Vec::new();

    let expr = calculator6::ExprsParser::new()
        .parse(&mut errors, "22 * + 3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * error) + 3)]");

    let expr = calculator6::ExprsParser::new()
        .parse(&mut errors, "22 * 44 + 66, *3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (error * 3)]");

    let expr = calculator6::ExprsParser::new()
        .parse(&mut errors, "*")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[(error * error)]");

    assert_eq!(errors.len(), 4);
}
```
