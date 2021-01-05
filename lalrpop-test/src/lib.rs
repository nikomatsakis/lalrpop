#![cfg_attr(not(test), allow(dead_code, unused_imports))]
#![allow(unused_doc_comments)]

extern crate diff;
#[macro_use]
extern crate lalrpop_util;

use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::process::Command;

use lalrpop_util::{ErrorRecovery, ParseError};

use crate::util::tok::Tok;

mod util;

/// Tests that actions can return the grammar's type parameters' associated
/// types.
lalrpop_mod!(associated_types);
mod associated_types_lib;

/// demonstration from the Greene text; one of the simplest grammars
/// that still ensures we get parse tree correct
lalrpop_mod!(sub);

/// test something other than test-all
lalrpop_mod!(sub_ascent);
lalrpop_mod!(sub_table);

/// more interesting demonstration of parsing full expressions
lalrpop_mod!(expr);

/// more interesting demonstration of parsing full expressions, using LALR not LR
lalrpop_mod!(expr_lalr);

/// more interesting demonstration of parsing full expressions, using intern tok
lalrpop_mod!(expr_intern_tok);

/// tests #![attributes] for generated module
lalrpop_mod!(
    #[allow(dead_code, unknown_lints)]
    expr_module_attributes
);

/// test that passes in lifetime/type/formal parameters and threads
/// them through, building an AST from the result
lalrpop_mod!(expr_arena);

/// definitions of the AST
mod expr_arena_ast;

/// expr defined with a generic type `F`
lalrpop_mod!(expr_generic);

lalrpop_mod!(generics_issue_104);
mod generics_issue_104_lib;

/// Grammar parameterized by `F` with where clause `where F: for<'a> FnMut(&'a
/// str)`.
lalrpop_mod!(where_clause_with_forall);

/// test of inlining
lalrpop_mod!(inline);

/// test that exercises internal token generation, as well as locations and spans
lalrpop_mod!(intern_tok);

/// test that exercises using a lifetime parameter in the token type
lalrpop_mod!(lifetime_tok);

/// library for lifetime_tok test
mod lifetime_tok_lib;

/// test that exercises locations and spans
lalrpop_mod!(loc);

/// regression test for location issue #90
lalrpop_mod!(loc_issue_90);
mod loc_issue_90_lib;

/// tests that user can use `<mut v:E+> <e:T> => { v.push(e); v }` instead of
/// `<v:E+> <e:T> => { let mut v = v; v.push(e); v }`
lalrpop_mod!(mut_name);

/// test that uses `super` in paths in various places
lalrpop_mod!(use_super);

mod pub_in;

/// Custom error type (issue #113)
#[derive(Debug, PartialEq)]
pub struct MyCustomError(char);

/// test that exercises locations, spans, and custom errors
lalrpop_mod!(error);
lalrpop_mod!(error_issue_113);

/// Test error recovery
lalrpop_mod!(error_recovery);
lalrpop_mod!(error_recovery_pull_182);
lalrpop_mod!(error_recovery_issue_240);
lalrpop_mod!(error_recovery_lalr_loop);
lalrpop_mod!(error_recovery_lock_in);
lalrpop_mod!(error_recovery_span);
lalrpop_mod!(
    #[allow(dead_code)]
    error_recovery_type_in_macro
);

/// test for inlining expansion issue #55
lalrpop_mod!(issue_55);

/// test for unit action code
lalrpop_mod!(unit);

/// test for match section
lalrpop_mod!(match_section);
lalrpop_mod!(match_section_byte);
lalrpop_mod!(match_alternatives);

/// regression test for issue #253.
lalrpop_mod!(partial_parse);

/// regression test for issue #278.
lalrpop_mod!(error_issue_278);

/// test for generic macros issue #417.
lalrpop_mod!(generics_issue_417);

lalrpop_mod!(
    #[deny(overflowing_literals)]
    #[allow(unused)]
    issue_394
);

lalrpop_mod!(
    // No parser should have been generated so nothing should be unused
    #[deny(dead_code)]
    cfg
);

lalrpop_mod!(
    #[deny(bare_trait_objects)]
    #[allow(unused)]
    dyn_argument
);

lalrpop_mod!(comments);

lalrpop_mod!(sp_from_optional);

lalrpop_mod!(nested);

pub fn use_cfg_created_parser() {
    cfg::CreatedParser::new();
}

/// This constant is here so that some of the generator parsers can
/// refer to it in order to test `super::` handling in action code.
const ZERO: i32 = 0;

#[test]
fn expr_test1() {
    util::test(|v| expr::ExprParser::new().parse(1, v), "22 - 3", 22 - 3);
}

#[test]
fn expr_test2() {
    util::test(
        |v| expr::ExprParser::new().parse(1, v),
        "22 - (3 + 5)",
        22 - (3 + 5),
    );
}

#[test]
fn expr_test3() {
    util::test(
        |v| expr::ExprParser::new().parse(1, v),
        "22 - (3 - 5) - 13",
        22 - (3 - 5) - 13,
    );
}

#[test]
fn expr_test4() {
    util::test(
        |v| expr::ExprParser::new().parse(1, v),
        "22 * 3 - 6",
        22 * 3 - 6,
    );
}

#[test]
fn expr_test5() {
    util::test(
        |v| expr::ExprParser::new().parse(11, v),
        "22 * 3 - 6",
        22 * 11 * 3 * 11 - 6 * 11,
    );
}

#[test]
fn expr_intern_tok_test1() {
    assert_eq!(
        expr_intern_tok::ExprParser::new()
            .parse(1, "22 - 3")
            .unwrap(),
        22 - 3
    );
}

#[test]
fn expr_intern_tok_test2() {
    assert_eq!(
        expr_intern_tok::ExprParser::new()
            .parse(1, "22 - (3 - 5) - 13")
            .unwrap(),
        22 - (3 - 5) - 13
    );
}

#[test]
fn expr_intern_tok_test_err() {
    match expr_intern_tok::ExprParser::new().parse(1, "22 - (3 - 5) - X") {
        //                                0123456789012345
        Err(ParseError::InvalidToken { location }) => {
            assert_eq!(location, 15);
        }
        r => {
            panic!("invalid result {:?}", r);
        }
    }
}

#[test]
fn parse_error_map_token_and_location() {
    let expr = "(1+\n(2++3))";
    let result = expr_intern_tok::ExprParser::new().parse(1, expr);
    let err: lalrpop_util::ParseError<usize, expr_intern_tok::Token, &'static str> =
        result.unwrap_err();

    let message = err
        .map_token(|expr_intern_tok::Token(_, t)| format!("TOKEN {}", t))
        .map_location(|offset| format!("line {}", expr[0..offset].lines().count()))
        .to_string();
    assert!(message.contains("Unrecognized token `TOKEN +`"));
    assert!(message.contains("at line 2"));
}

#[test]
fn parse_error_map_err() {
    let input = "---+";
    let err: lalrpop_util::ParseError<usize, util::tok::Tok<'static>, char> =
        util::test_err_gen(|t| error::ItemsParser::new().parse(t), input).unwrap_err();
    let modified_err = err.map_error(|c| c.to_string());
    if let ParseError::User {
        error: user_error_value,
    } = modified_err
    {
        assert_eq!(user_error_value, "+");
    } else {
        panic!("Expected a User error")
    }
}

#[test]
fn parse_error_eof_location() {
    match expr_intern_tok::ExprParser::new().parse(1, "1 - ") {
        Err(ParseError::UnrecognizedEOF { location, .. }) => {
            assert_eq!(location, 3);
        }
        _ => {
            panic!("Expected an UnrecognizedEOF error");
        }
    }
}

#[test]
fn display_parse_error() {
    let expr = "(1+\n(2++3))";
    let err = expr_intern_tok::ExprParser::new()
        .parse(1, expr)
        .unwrap_err();
    let message = err.to_string();
    assert!(message.contains("Unrecognized token `+`"));
}

#[test]
fn expr_lifetime_tok1() {
    // the problem here was that we were improperly pruning the 'input from the
    let tokens = lifetime_tok_lib::lt_tokenize("x");
    let tree = lifetime_tok::ExprParser::new().parse(tokens).unwrap();
    assert_eq!(tree, vec!["x"]);
}

#[test]
fn expr_lalr_test1() {
    util::test(
        |v| expr_lalr::ExprParser::new().parse(1, v),
        "22 - 3",
        22 - 3,
    );
}

#[test]
fn expr_lalr_test2() {
    util::test(
        |v| expr_lalr::ExprParser::new().parse(1, v),
        "22 - (3 + 5)",
        22 - (3 + 5),
    );
}

#[test]
fn expr_lalr_test3() {
    util::test(
        |v| expr_lalr::ExprParser::new().parse(1, v),
        "22 - (3 - 5) - 13",
        22 - (3 - 5) - 13,
    );
}

#[test]
fn expr_lalr_test4() {
    util::test(
        |v| expr_lalr::ExprParser::new().parse(1, v),
        "22 * 3 - 6",
        22 * 3 - 6,
    );
}

#[test]
fn expr_lalr_test5() {
    util::test(
        |v| expr_lalr::ExprParser::new().parse(11, v),
        "22 * 3 - 6",
        22 * 11 * 3 * 11 - 6 * 11,
    );
}

#[test]
fn inline_test1() {
    assert_eq!(inline::EParser::new().parse("& L L").unwrap(), "& L L");
}

#[test]
fn sub_test1() {
    util::test(|t| sub::SParser::new().parse(t), "22 - 3", 22 - 3);
}

#[test]
fn sub_test2() {
    util::test(
        |t| sub::SParser::new().parse(t),
        "22 - (3 - 5)",
        22 - (3 - 5),
    );
}

#[test]
fn sub_test3() {
    util::test(
        |t| sub::SParser::new().parse(t),
        "22 - (3 - 5) - 13",
        22 - (3 - 5) - 13,
    );
}

#[test]
fn sub_ascent_test1() {
    util::test(|t| sub_ascent::SParser::new().parse(t), "22 - 3", 22 - 3);
}

#[test]
fn sub_table_test1() {
    util::test(|t| sub_table::SParser::new().parse(t), "22 - 3", 22 - 3);
}

#[test]
fn expr_arena_test1() {
    use crate::expr_arena_ast::*;
    let arena = Arena::new();
    let expected = arena.alloc(Node::Binary {
        op: Op::Sub,
        l: arena.alloc(Node::Binary {
            op: Op::Mul,
            l: arena.alloc(Node::Value(22)),
            r: arena.alloc(Node::Value(3)),
        }),
        r: arena.alloc(Node::Value(6)),
    });
    util::test_loc(
        |v| expr_arena::ExprParser::new().parse(&arena, v),
        "22 * 3 - 6",
        expected,
    );
}

#[test]
fn expr_arena_test2() {
    use crate::expr_arena_ast::*;
    let arena = Arena::new();
    let expected = arena.alloc(Node::Reduce(
        Op::Mul,
        vec![
            arena.alloc(Node::Value(22)),
            arena.alloc(Node::Value(3)),
            arena.alloc(Node::Value(6)),
        ],
    ));
    util::test_loc(
        |v| expr_arena::ExprParser::new().parse(&arena, v),
        "*(22, 3, 6)",
        expected,
    );
    util::test_loc(
        |v| expr_arena::ExprParser::new().parse(&arena, v),
        "*(22, 3, 6,)",
        expected,
    );
}

#[test]
fn expr_arena_test3() {
    use crate::expr_arena_ast::*;
    let arena = Arena::new();
    let expected = arena.alloc(Node::Binary {
        op: Op::Mul,
        l: arena.alloc(Node::Value(22)),
        r: arena.alloc(Node::Paren(arena.alloc(Node::Binary {
            op: Op::Sub,
            l: arena.alloc(Node::Value(3)),
            r: arena.alloc(Node::Value(6)),
        }))),
    });
    util::test_loc(
        |v| expr_arena::ExprParser::new().parse(&arena, v),
        "22 * (3 - 6)",
        expected,
    );
}

#[test]
fn expr_generic_test1() {
    let expected: i32 = 22 * 3 - 6;
    let actual = expr_generic::ExprParser::new()
        .parse::<i32>("22 * 3 - 6")
        .unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn intern_tok_test1() {
    let expected = vec![
        (0, 0), // spans of `+` characters, measured in bytes
        (2, 3),
        (4, 5),
        (8, 9),
    ];
    let actual = intern_tok::ItemsParser::new().parse("--+-+---+").unwrap();
    //                                    012345678
    assert_eq!(actual, expected);
}

#[test]
fn loc_test1() {
    let expected = vec![
        (0, 0), // note that tok.rs generates odd spans, measured in 2*chars
        (4, 5),
        (8, 9),
        (16, 17),
    ];
    util::test_loc(|v| loc::ItemsParser::new().parse(v), "--+-+---+", expected);
    //                                       000001111
    //                                       024680246
}

#[test]
fn loc_test2() {
    util::test_loc(
        |v| loc::ItemsParser::new().parse(v),
        "+",
        vec![(0, 0), (0, 1)],
    );
}

#[test]
fn loc_empty() {
    // test what happens when `@L` and `@R` are invoked on an empty input
    util::test_loc(|v| loc::ItemsParser::new().parse(v), "", vec![(0, 0)]);
}

#[test]
fn use_super_test1() {
    util::test(|v| use_super::SParser::new().parse(v), "()", 0);
}

#[test]
fn error_test1() {
    match util::test_err_gen(|t| error::ItemsParser::new().parse(t), "---+") {
        Err(ParseError::User { error: '+' }) => { /* OK! */ }
        r => {
            panic!("unexpected response from parser: {:?}", r);
        }
    }
}

#[test]
fn error_recovery_eof() {
    let errors = RefCell::new(vec![]);
    util::test(
        |v| error_recovery::ItemParser::new().parse(&errors, v),
        "--",
        '!'.to_string(),
    );

    assert_eq!(errors.borrow().len(), 1);
    assert_eq!(
        errors.borrow()[0],
        ErrorRecovery {
            error: ParseError::UnrecognizedEOF {
                location: (),
                expected: vec!["\"-\"".to_string()],
            },
            dropped_tokens: vec![],
        }
    );
}

#[test]
fn error_recovery_eof_without_recovery() {
    let errors = RefCell::new(vec![]);
    let tokens = util::tok::tokenize("-").into_iter().map(|t| t.1);
    let result = error_recovery::ItemParser::new().parse(&errors, tokens);
    assert_eq!(
        result,
        Err(ParseError::UnrecognizedEOF {
            location: (),
            expected: vec!["\"-\"".to_string()],
        })
    );
}

#[test]
fn error_recovery_extra_token() {
    let errors = RefCell::new(vec![]);
    util::test(
        |v| error_recovery::ItemParser::new().parse(&errors, v),
        "(++)",
        "()".to_string(),
    );

    assert_eq!(errors.borrow().len(), 1);
    assert_eq!(
        errors.borrow()[0],
        ErrorRecovery {
            error: ParseError::UnrecognizedToken {
                token: ((), Tok::Plus, ()),
                expected: vec!["\")\"".to_string()],
            },
            dropped_tokens: vec![((), Tok::Plus, ())],
        }
    );
}

#[test]
fn error_recovery_dont_drop_unrecognized_token() {
    let errors = RefCell::new(vec![]);
    util::test(
        |v| error_recovery::ItemParser::new().parse(&errors, v),
        "(--)",
        "(!)".to_string(),
    );

    assert_eq!(errors.borrow().len(), 1);
    assert_eq!(
        errors.borrow()[0],
        ErrorRecovery {
            error: ParseError::UnrecognizedToken {
                token: ((), Tok::RParen, ()),
                expected: vec!["\"-\"".to_string()],
            },
            dropped_tokens: vec![],
        }
    );
}

#[test]
fn error_recovery_multiple_extra_tokens() {
    let errors = RefCell::new(vec![]);
    util::test(
        |v| error_recovery::ItemParser::new().parse(&errors, v),
        "(+++)",
        "()".to_string(),
    );

    assert_eq!(errors.borrow().len(), 1);
    assert_eq!(
        errors.borrow()[0],
        ErrorRecovery {
            error: ParseError::UnrecognizedToken {
                token: ((), Tok::Plus, ()),
                expected: vec!["\")\"".to_string()],
            },
            dropped_tokens: vec![((), Tok::Plus, ()), ((), Tok::Plus, ())],
        }
    );
}

#[test]
fn error_recovery_dont_panic_on_reduce_normal() {
    let mut errors = vec![];
    util::test(
        |v| error_recovery_pull_182::ItemParser::new().parse(&mut errors, v),
        "1+/",
        (),
    );

    assert_eq!(errors.len(), 1);
}

#[test]
fn error_recovery_dont_panic_on_reduce_at_eof() {
    let mut errors = vec![];
    util::test(
        |v| error_recovery_pull_182::ItemParser::new().parse(&mut errors, v),
        "1+",
        (),
    );

    assert_eq!(errors.len(), 1);
}

#[test]
fn error_recovery_issue_240() {
    let mut errors = vec![];

    // Test here is that error recovery should not go into an infinite loop. =)
    match util::test_err_gen(
        |v| error_recovery_issue_240::ExprParser::new().parse(&mut errors, v),
        "(+1/",
    ) {
        Ok(()) => {}
        r => {
            panic!("unexpected response from parser: {:?}", r);
        }
    }

    assert_eq!(
        errors,
        vec![ErrorRecovery {
            error: ParseError::UnrecognizedToken {
                token: (6, Tok::Div, 7),
                expected: vec!["\")\"".to_string()],
            },
            dropped_tokens: vec![(6, Tok::Div, 7)],
        },]
    );
}

#[test]
fn error_recovery_lalr_loop() {
    let mut errors = vec![];

    // In LALR (or Lane Table) mode, this was causing infinite loops
    // during recovery. We would drop tokens until EOF, but then get
    // ourselves in an error state where EOF was (ultimately) not
    // legal, triggering repeated error recovery. This is a variant of
    // the 'lock-in' phenomena discussed below.
    //
    // The newer algorithm is not so silly, however; when we get to EOF, we roll
    // back far enough that EOF becomes legal.
    match util::test_err_gen(
        |v| error_recovery_lalr_loop::ExprParser::new().parse(&mut errors, v),
        "(+1/",
    ) {
        Ok(()) => {
            assert_eq!(errors.len(), 1);
            let (l, _error, r) = errors.pop().unwrap();
            assert_eq!((l..r), (0..7)); // we popped everything, so this is the full string
        }
        r => {
            panic!("unexpected response from parser: {:?}", r);
        }
    }

    assert_eq!(errors, vec![]);
}

#[test]
fn error_recovery_lock_in() {
    let mut errors = vec![];

    // In this case, the `/` is in error, but we wind up dropping the
    // `(1` and calling that an error, so we effectively parse
    // `<error> / 22`.
    //
    // In the older strategy, in which we pop states to innermost
    // state, we used to recover after the `/` to a B (built from error). This
    // forces us to drop `/22` because `(B` can only be followed by
    // `)`.
    //
    // This is a good example of why it would be nice to give access
    // to the popped state.
    match util::test_err_gen(
        |v| error_recovery_lock_in::AParser::new().parse(&mut errors, v),
        "(1/22",
    ) {
        Ok(()) => {
            assert_eq!(errors.len(), 1); // should not drop any tokens
            let (
                l,
                ErrorRecovery {
                    error: _,
                    dropped_tokens,
                },
                r,
            ) = errors.pop().unwrap();
            assert_eq!((l..r), (0..3)); // span should cover the `(1` but not the `/`
            assert_eq!(dropped_tokens, vec![]);
        }
        r => {
            panic!("unexpected response from parser: {:?}", r);
        }
    }

    assert_eq!(errors, vec![]);
}

fn test_error_recovery_spans(input: &str, error_span: &str, output: &str) {
    let mut errors = vec![];

    let result = util::test_err_gen(
        |v| error_recovery_span::ExprParser::new().parse(&mut errors, v),
        input,
    )
    .unwrap();

    assert_eq!(result, output);

    // The "span locations" in our tests are a bit weird. We just
    // enumerate the tokens. So out input is broken into words
    // to help us simulate that.

    // Yields up pairs (start, end) for each non `.` token.  The
    // "start" half of the first one is considered the start of the
    // span, the "end" half of the second is the end of the span.
    let dash_tokens = || {
        error_span
            .split(char::is_whitespace)
            .enumerate()
            .filter_map(|(l, s)| {
                match s {
                    "." => None,

                    // Used to indicate that a span starts (or ends,
                    // if this is the last `-`) on this character.
                    "-" => Some((l * 2, l * 2 + 1)),

                    // Used to signal that a span starts just after
                    // this character.
                    ">" => Some((l * 2 + 1, l * 2 + 1)),

                    // Used to signal that a span ends just
                    // *before* this character.
                    "<" => Some((l * 2, l * 2)),

                    _ => panic!("invalid character in span"),
                }
            })
    };

    let lo = dash_tokens().next().unwrap().0;
    let hi = dash_tokens().last().unwrap().1;
    assert_eq!(vec![(lo, hi)], errors);
}

#[test]
fn error_recovery_span_starts_from_dropped_state() {
    // Span starts from dropped state `+`
    // Span ends after dropped token `/`
    test_error_recovery_spans("1 + ( 2 + / )", ". . . . - - .", "1 + (2)");
}

#[test]
fn error_recovery_span_starts_from_initial() {
    // Span starts (and ends) before the `+`.
    test_error_recovery_spans("+ 3", "< .", "! + 3");
}

#[test]
fn error_recovery_span_starts_just_dropped_tokens() {
    // We retain the `3`, but drop the `4`; our span
    // covers just the dropped token (`4`).
    test_error_recovery_spans("1 + ( 2 + 3 4 )", ". . . . . . - .", "1 + (2 + 3)");
}

#[test]
fn error_recovery_span_starts_inserted_tokens() {
    // We neither drop states *nor* drop tokens, but *insert* a `!`
    // into the stream so that `( ! )` can reduce.
    //
    // That is, we use the end of the top-most state
    // as our start and the start of the lookahead as our
    // end.
    test_error_recovery_spans("1 + ( )", ". . > <", "1 + (!)");
}

#[test]
fn error_recovery_span_starts_inserted_token_eof() {
    // We insert a ! before EOF, neither dropped states nor tokens.
    test_error_recovery_spans("1 -", ". >", "1 -!");
}

#[test]
fn error_recovery_span_starts_just_dropped_states() {
    // Here, we drop the `+` in favor of the `-`.
    // Therefore, the span we give is the `+`.
    test_error_recovery_spans("1 + - 4", ". - . .", "1 - 4");
}

#[test]
fn issue_55_test1() {
    // Issue 55 caused us to either accept NO assoc types or assoc
    // types both before and after, so check that we can parse with
    // assoc types on either side.

    let (a, b, c) = issue_55::EParser::new()
        .parse("{ type X; type Y; enum Z { } }")
        .unwrap();
    assert_eq!(a, vec!["X", "Y"]);
    assert_eq!(b, "Z");
    assert!(c.is_empty());

    let (a, b, c) = issue_55::EParser::new()
        .parse("{ enum Z { } type X; type Y; }")
        .unwrap();
    assert!(a.is_empty());
    assert_eq!(b, "Z");
    assert_eq!(c, vec!["X", "Y"]);
}

#[test]
fn unit_test1() {
    assert!(unit::ExprParser::new().parse("3 + 4 * 5").is_ok());
}

#[test]
fn unit_test2() {
    assert!(unit::ExprParser::new().parse("3 + +").is_err());
}

#[test]
fn generics_issue_104_test1() {
    // The real thing `generics_issue_104` is testing is that the code
    // *compiles*, even though the type parameter `T` does not appear
    // in any of the arguments.
    assert!(generics_issue_104::SchemaParser::new()
        .parse::<()>("grammar { foo }")
        .is_ok());
}

#[test]
fn where_clause_with_forall_test1() {
    assert!(where_clause_with_forall::TermParser::new()
        .parse(&mut |s: &str| println!("log: {}", s), "(((((42)))))")
        .is_ok());
}

#[test]
fn test_match_section() {
    assert!(match_section::QueryParser::new()
        .parse("SELECT foo")
        .is_ok());
    assert!(match_section::QueryParser::new()
        .parse("select foo")
        .is_ok());
    assert!(match_section::QueryParser::new()
        .parse("INSERT foo")
        .is_ok());
    assert!(match_section::QueryParser::new()
        .parse("UPDATE foo")
        .is_ok());
    assert!(match_section::QueryParser::new()
        .parse("UPDATE update")
        .is_err());
}

#[test]
fn test_match_section_byte() {
    assert!(match_section::QueryParser::new()
        .parse("SELECT foo")
        .is_ok());
    assert!(match_section::QueryParser::new()
        .parse("select foo")
        .is_ok());
    assert!(match_section::QueryParser::new()
        .parse("INSERT foo")
        .is_ok());
    assert!(match_section::QueryParser::new()
        .parse("UPDATE foo")
        .is_ok());
    assert!(match_section::QueryParser::new()
        .parse("UPDATE update")
        .is_err());
}

#[test]
fn test_match_alternatives() {
    assert_eq!(
        match_alternatives::FileParser::new().parse("foo true"),
        Ok("foo true".to_string())
    );
    assert_eq!(
        match_alternatives::FileParser::new().parse("bar false"),
        Ok("bar false".to_string())
    );
}

#[test]
fn test_mut_name() {
    assert_eq!(
        mut_name::MutParser::new().parse("1, 2, 3: 4"),
        Ok(vec![1, 2, 3, 4])
    );
}

#[test]
fn issue_113() {
    assert!(error_issue_113::ItemsParser::new().parse("+").is_err());
}

#[test]
fn issue_253() {
    assert!(partial_parse::TermParser::new().parse("(22))").is_err());
}

#[test]
fn test_action_return_associated_types() {
    let mut callbacks = associated_types_lib::TestParseCallbacks;
    assert_eq!(
        associated_types::TermParser::new().parse(&mut callbacks, "(((((42)))))"),
        Ok(associated_types_lib::TestTerm(
            associated_types_lib::TestNum(42)
        ))
    );
}

#[test]
fn error_issue_278() {
    match error_issue_278::ValueParser::new().parse("123 abc") {
        Err(ParseError::User {
            error: "Pretend there was an error",
        }) => { /* OK! */ }
        r => {
            panic!("unexpected response from parser: {:?}", r);
        }
    }
}

#[test]
fn generics_issue_417() {
    // Similar to `generics_issue_104`
    // The real thing `generics_issue_417` is testing is that the code
    // *compiles*, even though the type parameter `T` does not appear
    // in any of the arguments.
    assert!(generics_issue_417::TupleParser::new()
        .parse::<()>("(hello, world)")
        .is_ok());
}

#[test]
fn verify_lalrpop_generates_itself() {
    let out_dir = "../target";
    let lrgrammar = "lrgrammar.lalrpop";
    let grammar_file = Path::new("../lalrpop/src/parser/").join(lrgrammar);
    let copied_grammar_file = Path::new(out_dir).join(lrgrammar);

    // Don't remove the .rs file that already exist
    fs::copy(&grammar_file, &copied_grammar_file).expect("no grammar file found");

    assert!(Command::new("../target/debug/lalrpop")
        .args(&[
            "--force",
            "--no-whitespace",
            "--out-dir",
            out_dir,
            copied_grammar_file
                .to_str()
                .expect("grammar path is not UTF-8")
        ])
        .status()
        .expect("lalrpop run failed")
        .success());

    let actual = fs::read_to_string(grammar_file.with_extension("rs")).unwrap();
    let expected = fs::read_to_string(copied_grammar_file.with_extension("rs")).unwrap();
    util::compare_str(
        &actual,
        &expected,
        "The snapshot does not match what lalrpop generates now.\n\
         Use ./snap.sh to generate a new snapshot of the lrgrammar",
    );
}

#[test]
fn comments() {
    assert_eq!(
        comments::TermParser::new().parse("22 3 5 13").unwrap(),
        vec!["22", "3", "5", "13"]
    );

    assert_eq!(
        comments::TermParser::new()
            .parse(
                "22 /* 123 */ 3 5
            //  abc
            13 // "
            )
            .unwrap(),
        vec!["22", "3", "5", "13"]
    );
}

#[test]
fn sp_from_optional() {
    assert_eq!(
        sp_from_optional::TestParser::new()
            .parse("before   let")
            .unwrap(),
        (9, "let", 12)
    );
}

#[test]
fn test_nested_pattern() {
    let tokens = util::tok::tokenize("{{]").into_iter().map(|t| t.1);
    assert_eq!(nested::EParser::new().parse(tokens.into_iter()).unwrap(), 1);
}

#[test]
fn test_string_tokenize() {
    let tokens = util::tok::tokenize("1 \"just testing\" 2")
        .into_iter()
        .map(|t| t.1)
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![Tok::Num(1), Tok::String("just testing"), Tok::Num(2)]
    );
}

#[test]
fn test_nested_pattern_string() {
    let tokens = util::tok::tokenize("{{]\"hello\"").into_iter().map(|t| t.1);
    assert_eq!(
        nested::EParser::new().parse(tokens.into_iter()).unwrap(),
        101
    );
}

#[test]
fn test_nested_pattern_string_error() {
    let tokens = util::tok::tokenize("\"not matched\"")
        .into_iter()
        .map(|t| t.1);
    let err = nested::EParser::new()
        .parse(tokens.into_iter())
        .unwrap_err();
    match err {
        ParseError::UnrecognizedToken { token, expected: _ } => {
            assert_eq!(token.1, Tok::String("not matched"));
        }
        _ => {
            panic!("Unexpected error: {:?}", err);
        }
    }
}
