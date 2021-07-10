This is a tutorial for how to write a complete parser for a simple calculator using LALRPOP.

If you are unfamiliar with what a parser generator is, you should read [Crash course on parsers]
first.

- [Adding LALRPOP to your project](001_adding_lalrpop.html)
- [Parsing parenthesized numbers](002_paren_numbers.html)
- [Type inference](003_type_inference.html)
- [Handling full expressions](004_full_expressions.html)
- [Building ASTs](005_building_asts.html)
- [Macros](006_macros.html)
- [Fallible actions](007_fallible_actions.html)
- [Error recovery](008_error_recovery.html)
- [Passing state parameter](009_state_parameter.html)

This tutorial is still incomplete. Here are some topics that I aim to
cover when I get time to write about them:

- Advice for resolving shift-reduce and reduce-reduce conflicts
- Passing state and type/lifetime parameters to your action code (see e.g. [this test](https://github.com/lalrpop/lalrpop/blob/master/lalrpop-test/src/expr_arena.lalrpop) invoked [from here]).
- Location tracking with `@L` and `@R` (see e.g. [this test](https://github.com/lalrpop/lalrpop/blob/master/lalrpop-test/src/intern_tok.lalrpop)).
- Integrating with external tokenizers (see e.g. [this test](https://github.com/lalrpop/lalrpop/blob/master/lalrpop-test/src/expr.lalrpop) invoked [from here]).
- Conditional macros (no good test to point you at yet, sorry)
- Fallible action code that produces a `Result` (see e.g. [this test](https://github.com/lalrpop/lalrpop/blob/master/lalrpop-test/src/error.lalrpop) invoked [from here]).
- Converting to use `LALR(1)` instead of `LR(1)` (see e.g. [this test](https://github.com/lalrpop/lalrpop/blob/master/lalrpop-test/src/expr_lalr.lalrpop) invoked [from here]).
- Plans for future features

[Crash course on parsers]: ../crash_course.md
[from here]: https://github.com/lalrpop/lalrpop/blob/master/lalrpop-test/src/lib.rs
