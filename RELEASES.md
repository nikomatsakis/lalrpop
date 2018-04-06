# Version 0.15.1

Fixes:
- Don't overflow the stack in parse table debug builds (#337) 
- Use the correct type for `!` in macro expanded productions (#335)
- Allow lalrpop parsers to be used with include! (#338)
- Remove dependency on docopt, rustc-serialize, update itertools  (#344, #345)
- Correctly anchor regex at the beginning (#358)

Thanks to the following contributors for this release:
- @Marwes
- @mbrubek
- @waywardmonkeys
- @sanxiyn
- @17cupsofcoffee
- @matklad

# Version 0.15

Features:

- The source and binary size of generated parsers has been reduced (#324, #306)
- Regex compilation as part of the generated lexer can now be cached (#318)
- The documentation is now provided as a [mbbook](http://lalrpop.github.io/lalrpop/) (#298)

Bugs fixed:

- Fixed a stack overflow in debug builds of large grammars (#337)
- The error terminal now gets the correct type assigned when part of macros (#335)
- Character literals now parse correctly in the parser files (#320)

Compatibility notes:

- To let regex compilation be cached, each parser are now generated as a struct
  with a `parse` method instead of just a function.
  To upgrade, change each parse call from `parse_X(..)` to `X::new().parse(..)`.

Thanks to the following contributors for this release:

- @Phlosioneer
- @waywardmonkeys
- @brendanzab
- @dtkerr
- @Marwes
- @ahmedcharles
- @udoprog

# Version 0.14

Bugs fixed:

- Infinite loop in error recovery fixed (#240).
- Bad error messages if a `;` was forgotten fixed (#276).
- Grammar errors were sometimes incorrectly reported as "extra tokens" (#278)
- `extern` blocks now allowed even when not using a custom tokenizer (#261)
- `ParseError` now implements `Display`
- actions can now return a grammar's type parameter's associated type (#247)
- generated files are now rebuilt when there is a new LALRPOP version (#243)

Compatibility notes:

- As part of making `ParseError` implement `Display`, the default
  error type changed from `()` to `&'static str`, so parse errors type
  may change from `lalrpop_util::ParseError<..., ()>` to
  `lalrpop_util::ParseError<..., &'static str>`.

Thanks to the following contributors for this release:

- @fitzgen
- @joerivanruth
- @pyfisch
- @nick70
- @notriddle
- @vmx

# Version 0.13.1

- We now support `#![..]` attributes in `.lalrpop` files.
- We now use lane table by default: since the lane table algorithm
  automatically generates compressed tables where possible, the
  `#[lalr]` attribute is still accepted, but has no effect.
  - If you encounter problems, please report bugs! In the meantime,
    though, you can use the `LALRPOP_LANE_TABLE=disabled` environment
    variable to change back.
- When the `<>` string is found within `{}` inside of an action, it
  now generates a series of `x: x` pairs for each named value `x`.
  This is useful for struct constants, since you can do something
  like: `<a:Foo> <b:Bar> => MyStruct { <> }`, if `MyStruct` had two
  fields named `a` and `b`.
- We now support character literal patterns in the external tokenizer pattern syntax.  
- The lalrpop executable now supports `--version`.
- We are (for now, at least) testing for compatibility with Rust 1.13.
  This minimal supported rustc version may change in the future,
  however.
- Misc bug fixes.

Thanks to the following contributors for this release:

- @jchlapinski
- @minijackson
- @nikomatsakis
- @ravenexp
- @ruuda
- @wieczyk
- @withoutboats

# Version 0.13

This is a bug release for LALRPOP. First, we have two major improvements to the
generated lexer:

- The lexer now generates code that uses the `regex` crate. This
  results in **far** less code than the older style, and seems to
  preserve performance.
- The lexer now supports custom priorities for regular expression tokens,
  making it possible to support case-insensitive keywords.
  - See the [calculator2b example] for details.
  
Second, we have a **beta release** of the new lane-table based
LR-table generation.  Lane tables handle the full set of LR(1)
grammars but typically reduce **much** smaller state tables. This
feature eliminates the need to manually mark grammars as `#[LALR]`.
Lane tables are **not** on by default; you can enable them by setting
`LALRPOP_LANE_TABLE=enabled` in your environment (use
`std::env::set_var` in your `build.rs`).

[calculator2b example]: https://github.com/nikomatsakis/lalrpop/blob/master/doc/tutorial.md#calculator2b

Finally, the `lalrpop` executable now has the ability to generate
standalone reports (`--report`).

Fixed bugs:

- Fix #157: We now recognize single quote (`'`) properly in our tokenizer.
- Fix #179: Fix bug in recursive ascent code generation.

Thanks to the following contributors to this release:

- @ahmedcharles
- @king6cong
- @nikomatsakis
- @nixpulvis
- @wagenet
- @wieczyk

# Version 0.12.5

- Add the expected successor tokens to `UnrecognizedToken` errors ([thanks @Marwes!](https://github.com/nikomatsakis/lalrpop/pull/178)).
- Fix to error recovery doing a bad cast ([thanks @Marwes!](https://github.com/nikomatsakis/lalrpop/pull/182)).

# Version 0.12.4

Major new feature! @Marwes
[added support for error recovery](https://github.com/nikomatsakis/lalrpop/pull/160).

There have also been a number of other improvements:

- The `ParseError` type now implements `Error` and `Display` ([thanks @Marwes!](https://github.com/nikomatsakis/lalrpop/pull/167)).
- We no longer emit comments in generated code by default ([thanks @Marwes!](https://github.com/nikomatsakis/lalrpop/pull/165)).

# Versions 0.12.2, 0.12.3

(Yanked due to minor backwards incompatibility.)

# Version 0.12.1

Bug fix release. Major bugs addressed:

- ["LR1 TLS not installed"](https://github.com/nikomatsakis/lalrpop/issues/145);
- [False conflicts in the LR(1) code generator](https://github.com/nikomatsakis/lalrpop/issues/144).

Also, there is now a tutorial for writing custom lexers.
[Thanks @malleusinferni!](https://github.com/nikomatsakis/lalrpop/pull/131).

# Version 0.12 

**Enabled a new table-driven code-generator by default.** This generates
less code than the older recursive-ascent-based generation scheme, but
may parse less efficiently. To go back to the old scheme, annotate
the grammar declaration:

```
#[recursive_ascent] grammar;
```

Also, **the syntax for requesting LALR-generation has changed** to use
an annotation:

```
#[LALR] grammar;
```

We no longer **emit module-level attributes**, which means that unused
imports in your .lalrpop file may start getting warnings. [Thanks
@dflemstr!](https://github.com/nikomatsakis/lalrpop/pull/118)

An overflow bug in LALRPOP was
fixed. [Thanks @larsluthman!](https://github.com/nikomatsakis/lalrpop/pull/120)

We no longer depend on `time`, but now use
`std::time`. [Thanks @serprex!](https://github.com/nikomatsakis/lalrpop/pull/121)

There is now a `Configuration` object for use in your `build.rs`
scripts.  And,
[thanks to @dflemstr!](https://github.com/nikomatsakis/lalrpop/pull/108),
it permits you to configure the directory where LALRPOP output is
generated.

Fixed a bug in the LALRPOP option
parsing. [Thanks @Nemikolh!](https://github.com/nikomatsakis/lalrpop/pull/106)

Various typos and small corrections. Thanks [@reuben](https://github.com/nikomatsakis/lalrpop/pull/1103) and [@ashleygwilliams](https://github.com/nikomatsakis/lalrpop/pull/116)!

# Version 0.11

Updated to use the `regex-syntax` crate for regular expression
parsing instead of rolling our own parser. This means we can now
support the [same regular expression syntax as the regex crate](https://doc.rust-lang.org/regex/regex/index.html#syntax),
and in particular can support unicode character classes like `\p{Greek}`.
Note that some regex features -- such as non-greedy repetition and
named capture groups -- are still not supported (or just not meaningful).

Optimized LR(1) construction time by approximately 5x.

Improved handling of location tokens `@L` and `@R` so that they can be
freely used without ever causing parse conflicts.

# Version 0.10

Major update to LALRPOP error messages in cases of shift/reduce and
reduce/reduce conflicts. The messages now try to explain the problem
in terms of your grammar, as well as diagnosing common problem
scenarios and suggesting solutions.

Added a standalone LALRPOP executable.

We no longer generate incomplete files when grammar generation fails
(Issue #57).

# Version 0.9

Miscellaneous bug fixes, mostly. Processing for a `build.rs` file now
starts from the project directory, rather than being hardcoded to
start from `src`.

# Version 0.8

Add support for inlining nonterminals. Nonterminals can now be
annotated with `#[inline]`. If you do so, each use of the nonterminal
will be inlined into its place. This can be very helpful in addressing
shift-reduce or reduce-reduce conflicts, at the cost of a larger
grammar. We now inline `Foo*`, `Foo?`, and `(Foo Bar)` nonterminals by
default.

# Version 0.7

This is mostly a bug-fix release.

Various minor issues were addressed:

- Issue #25: Unbalanced parens in string literals appearing in code now work properly.
- Issue #32: Regular expression parsing consumed infinite memory when a `.` appeared.
- Issue #34: Automatic tokenizer generation did not play well with generic type parameters.

# Older versions

I hadn't yet started writing release notes, sorry.
