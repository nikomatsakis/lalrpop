# Building ASTs

Of course, most of the time, when you're parsing you don't want to
compute a value, you want to build up some kind of data structure.
Here's a quick example to show how that is done in LALRPOP.  First, we
need to *define* the data structure we will build. We're going to use
a very simple `enum`:

```rust
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}
```

We put this code into [an `ast.rs` module][astrs] in our project,
along with some `Debug` impls so that things pretty-print nicely. Now
we will create the [calculator4] example, which will build up this
tree. To start, let's just look at the `Expr` nonterminal, which will
show you most everything of how it is done (the most interesting lines
have been flagged with comments):

```lalrpop
use std::str::FromStr;
use ast::{Expr, Opcode}; // (0)

grammar;

pub Expr: Box<Expr> = { // (1)
    Expr ExprOp Factor => Box::new(Expr::Op(<>)), // (2)
    Factor,
};

ExprOp: Opcode = { // (3)
    "+" => Opcode::Add,
    "-" => Opcode::Sub,
};
```

First off, we have to import these new names into our file by adding a
`use` statement (0). Next, we want to produce `Box<Expr>` values, so
we change the type of `Expr` (and `Factor` and `Term`) to `Box<Expr>`
(1). The action code changes accordingly in (2); here we've used the
`<>` expansion to supply three arguments to `Expr::Op`. Finally, just
for concision, we introduced an `ExprOp` nonterminal (3) to cover the
two opcodes, which now trigger the same action code (before they
triggered different action code, so we could do an addition vs a
subtraction).

The definition of `Factor` is transformed in a similar way:

```lalrpop
Factor: Box<Expr> = {
    Factor FactorOp Term => Box::new(Expr::Op(<>)),
    Term,
};

FactorOp: Opcode = {
    "*" => Opcode::Mul,
    "/" => Opcode::Div,
};
```

And finally we adjust the definitions of `Term` and `Num`. Here, we
convert from a raw `i32` into a `Box<Expr>` when we transition from
`Num` to `Term` (4):

```lalrpop
Term: Box<Expr> = {
    Num => Box::new(Expr::Number(<>)), // (4)
    "(" <Expr> ")"
};

Num: i32 = {
    r"[0-9]+" => i32::from_str(<>).unwrap()
};
```

And that's it! Now we can test it by adding some code to our
[main.rs][main] file that parses an expression and formats it using
the `Debug` impl:

```rust
pub mod calculator4;
pub mod ast;

#[test]
fn calculator4() {
    let expr = calculator4::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}
```

[main]: calculator/src/main.rs
[calculator4]: calculator/src/calculator4.lalrpop
[astrs]: calculator/src/ast.rs
