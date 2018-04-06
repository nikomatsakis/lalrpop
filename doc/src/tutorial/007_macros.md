# Macros

Frequently when writing grammars we encounter repetitive constructs
that we would like to copy-and-paste. A common example is defining
something like a "comma-separated list". Imagine we wanted to parse a
comma-separated list of expressions (with an optional trailing comma,
of course).  If we had to write this out in full, it would look
something like:

```lalrpop
Exprs: Vec<Box<Expr>> = {
    Exprs "," Expr => ...,
    Expr => vec![<>],
}
```

Of course, this doesn't handle trailing commas, and I've omitted the
action code. If we added those, it would get a bit more
complicated. So far, this is fine, but then what happens when we later
want a comma-separated list of terms? Do we just copy-and-paste
everything?

LALRPOP offers a better option. You can define macros. In fact,
LALRPOP comes with four macros builtin: `*`, `?`, `+`, and `(...)`. So
you can write something like `Expr?` to mean "an optional
`Expr`". This will have type `Option<Box<Expr>>` (since `Expr` alone
has type `Box<Expr>`).  Similarly, you can write `Expr*` or `Expr+` to
get a `Vec<Expr>` (with minimum length 0 and 1 respectively). The
final macro is parentheses, which is a shorthand for creating a new
nonterminal.  This lets you write things like `(<Expr> ",")?` to mean
an "optionally parse an `Expr` followed by a comma". Note the angle
brackets around `Expr`: these ensures that the value of the `(<Expr>
",")` is the value of the expression, and not a tuple of the
expression and the comma. This means that `(<Expr> ",")?` would have
the type type `Option<Box<Expr>>` (and not `Option<(Box<Expr>, &'input
str)>`).

Using these operations we can define `Exprs` in terms of a macro
`Comma<T>` that creates a comma-separated list of `T`, whatever `T` is
(this definition appears in [calculator5]):

```lalrpop
pub Exprs = Comma<Expr>; // (0)

Comma<T>: Vec<T> = { // (1)
    <v:(<T> ",")*> <e:T?> => match e { // (2)
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
};
```

The definition of `Exprs` on line (0) is fairly obvious, I think. It
just uses a macro `Comma<Expr>`. Let's take a look then at the
definition of `Comma<T>` on line (1). This is sort of dense, so let's
unpack it. First, `T` is some terminal or nonterminal, but note that
we can also use it as a type: when the macro is expanded, the `T` in
the type will be replaced with "whatever the type of `T` is".

Next, on (2), we parse `<v:(<T> ",")*> <e:T?>`.  That's a lot of
symbols, so let's first remove all the angle brackets, which just
serve to tell LALRPOP what values you want to propagate and which you
want to discard. In that case, we have: `(T ",")* T?`. Hopefully you
can see that this matches a comma-separated list with an optional
trailing comma. Now let's add those angle-brackets back in. In the
parentheses, we get `(<T> ",")*` -- this just means that we keep the
value of the `T` but discard the value of the comma when we build our
vector. Then we capture that vector and call it `v`: `<v:(<T> ",")*>`.
Finally, we capture the optional trailing element `e`: `<e:T?>`. This
means the Rust code has two variables available to it, `v: Vec<T>` and
`e: Option<T>`. The action code itself should then be fairly clear --
if `e` is `Some`, it appends it to the vector and returns the result.

As another example of using macros, you may recall the precedence
tiers we saw in [calculator4] (`Expr`, `Factor`, etc), which had a
sort of repetitive structure. You could factor that out using a
macro. In this case, it's a recursive macro:

```lalrpop
Tier<Op,NextTier>: Box<Expr> = {
    Tier<Op,NextTier> Op NextTier => Box::new(Expr::Op(<>)),
    NextTier
};

Expr = Tier<ExprOp, Factor>;
Factor = Tier<FactorOp, Term>;

ExprOp: Opcode = { // (3)
    "+" => Opcode::Add,
    "-" => Opcode::Sub,
};

FactorOp: Opcode = {
    "*" => Opcode::Mul,
    "/" => Opcode::Div,
};
```

And, of course, we have to add some tests to [main.rs file][main]:

```rust
pub mod calculator5;

#[test]
fn calculator5() {
    let expr = calculator5::ExprsParser::new().parse("").unwrap();
    assert_eq!(&format!("{:?}", expr), "[]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66)]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66,")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66)]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66, 13*3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (13 * 3)]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66, 13*3,")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (13 * 3)]");
}
```

[main]: calculator/src/main.rs
[calculator4]: calculator/src/calculator4.lalrpop
[calculator5]: calculator/src/calculator5.lalrpop
