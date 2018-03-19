// auto-generated: "lalrpop 0.14.0"
#![cfg_attr(rustfmt, rustfmt_skip)]
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Term {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    pub struct TermParser {
        builder: super::__intern_token::__MatcherBuilder,
        _priv: (),
    }

    impl TermParser {
        pub fn new() -> TermParser {
            let __builder = super::__intern_token::__MatcherBuilder::new();
            TermParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<String, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let _ = self.builder;
            let __ascent = __ascent::TermParser::new().parse(
                input,
            );
            let __parse_table = __parse_table::TermParser::new().parse(
                input,
            );
            assert_eq!(__ascent, __parse_table);
            return __ascent;
        }
    }
    mod __ascent {

        mod __parse__Term {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

            #[allow(unused_extern_crates)]
            extern crate lalrpop_util as __lalrpop_util;
            use super::super::super::__intern_token::Token;
            pub struct TermParser {
                builder: super::super::super::__intern_token::__MatcherBuilder,
                _priv: (),
            }

            impl TermParser {
                pub fn new() -> TermParser {
                    let __builder = super::super::super::__intern_token::__MatcherBuilder::new();
                    TermParser {
                        builder: __builder,
                        _priv: (),
                    }
                }

                #[allow(dead_code)]
                pub fn parse<
                    'input,
                >(
                    &self,
                    input: &'input str,
                ) -> Result<String, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
                {
                    let mut __tokens = self.builder.matcher(input);
                    let __lookahead = match __tokens.next() {
                        Some(Ok(v)) => Some(v),
                        None => None,
                        Some(Err(e)) => return Err(e),
                    };
                    match try!(__state0(input, &mut __tokens, __lookahead, ::std::marker::PhantomData::<()>)) {
                        (Some(__lookahead), _) => {
                            Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead })
                        }
                        (None, __Nonterminal::____Term((_, __nt, _))) => {
                            Ok(__nt)
                        }
                        _ => unreachable!(),
                    }
                }
            }

            #[allow(dead_code)]
            pub enum __Nonterminal<>
             {
                Num((usize, String, usize)),
                Term((usize, String, usize)),
                ____Term((usize, String, usize)),
            }

            // State 0
            //     AllInputs = []
            //     OptionalInputs = []
            //     FixedInputs = []
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = None
            //
            //     Num = (*) r#"[0-9]+"# ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //     Term = (*) Num ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //     Term = (*) "(" Term ")" ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //     Term = (*) "22" ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //     Term = (*) r#"\\w+"# ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //     __Term = (*) Term ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //
            //   "(" -> S3
            //   "22" -> S4
            //   r#"[0-9]+"# -> S5
            //   r#"\\w+"# -> S6
            //
            //     Num -> S1
            //     Term -> S2
            fn __state0<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, Token<'input>, usize),__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Token<'input>, usize)>,
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Token<'input>, usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
            {
                let mut __result: (Option<(usize, Token<'input>, usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, Token(2, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state3(input, __tokens, __sym0, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Token(4, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state4(input, __tokens, __sym0, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Token(1, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(input, __tokens, __sym0, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Token(0, __tok0), __loc2)) => {
                        let __sym0 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(input, __tokens, __sym0, ::std::marker::PhantomData::<()>));
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###""(""###.to_string(),
                                r###""22""###.to_string(),
                                r###"r#"[0-9]+"#"###.to_string(),
                                r###"r#"\\w+"#"###.to_string(),
                            ]
                        });
                    }
                }
                loop {
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::Num(__sym0) => {
                            __result = try!(__state1(input, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym0) => {
                            __result = try!(__state2(input, __tokens, __lookahead, __sym0, ::std::marker::PhantomData::<()>));
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 1
            //     AllInputs = [Num]
            //     OptionalInputs = []
            //     FixedInputs = [Num]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = Num (*) ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //
            //   [")", EOF] -> Term = Num => ActionFn(1);
            //
            fn __state1<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, Token<'input>, usize),__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Token<'input>, usize)>,
                __sym0: (usize, String, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Token<'input>, usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
            {
                let mut __result: (Option<(usize, Token<'input>, usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((_, Token(3, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action1::<>(input, __sym0);
                        let __nt = __Nonterminal::Term((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###"")""###.to_string(),
                            ]
                        });
                    }
                }
            }

            // State 2
            //     AllInputs = [Term]
            //     OptionalInputs = []
            //     FixedInputs = [Term]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(__Term)
            //
            //     __Term = Term (*) ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //
            //   [EOF] -> __Term = Term => ActionFn(0);
            //
            fn __state2<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, Token<'input>, usize),__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Token<'input>, usize)>,
                __sym0: (usize, String, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Token<'input>, usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
            {
                let mut __result: (Option<(usize, Token<'input>, usize)>, __Nonterminal<>);
                match __lookahead {
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<>(input, __sym0);
                        let __nt = __Nonterminal::____Term((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                            ]
                        });
                    }
                }
            }

            // State 3
            //     AllInputs = ["("]
            //     OptionalInputs = []
            //     FixedInputs = ["("]
            //     WillPushLen = 2
            //     WillPush = [Term, ")"]
            //     WillProduce = Some(Term)
            //
            //     Num = (*) r#"[0-9]+"# ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //     Term = (*) Num ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //     Term = (*) "(" Term ")" ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //     Term = "(" (*) Term ")" ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //     Term = (*) "22" ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //     Term = (*) r#"\\w+"# ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //
            //   "(" -> S3
            //   "22" -> S4
            //   r#"[0-9]+"# -> S5
            //   r#"\\w+"# -> S6
            //
            //     Num -> S1
            //     Term -> S7
            fn __state3<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, Token<'input>, usize),__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Token<'input>, usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
            {
                let mut __result: (Option<(usize, Token<'input>, usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((__loc1, Token(2, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state3(input, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Token(4, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state4(input, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Token(1, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state5(input, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    Some((__loc1, Token(0, __tok0), __loc2)) => {
                        let __sym1 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state6(input, __tokens, __sym1, ::std::marker::PhantomData::<()>));
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###""(""###.to_string(),
                                r###""22""###.to_string(),
                                r###"r#"[0-9]+"#"###.to_string(),
                                r###"r#"\\w+"#"###.to_string(),
                            ]
                        });
                    }
                }
                loop {
                    let (__lookahead, __nt) = __result;
                    match __nt {
                        __Nonterminal::Num(__sym1) => {
                            __result = try!(__state1(input, __tokens, __lookahead, __sym1, ::std::marker::PhantomData::<()>));
                        }
                        __Nonterminal::Term(__sym1) => {
                            __result = try!(__state7(input, __tokens, __lookahead, __sym0, __sym1, ::std::marker::PhantomData::<()>));
                            return Ok(__result);
                        }
                        _ => {
                            return Ok((__lookahead, __nt));
                        }
                    }
                }
            }

            // State 4
            //     AllInputs = ["22"]
            //     OptionalInputs = []
            //     FixedInputs = ["22"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = "22" (*) ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //
            //   [")", EOF] -> Term = "22" => ActionFn(3);
            //
            fn __state4<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, Token<'input>, usize),__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Token<'input>, usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
            {
                let mut __result: (Option<(usize, Token<'input>, usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, Token(3, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3::<>(input, __sym0);
                        let __nt = __Nonterminal::Term((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###"")""###.to_string(),
                            ]
                        });
                    }
                }
            }

            // State 5
            //     AllInputs = [r#"[0-9]+"#]
            //     OptionalInputs = []
            //     FixedInputs = [r#"[0-9]+"#]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Num)
            //
            //     Num = r#"[0-9]+"# (*) ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //
            //   [")", EOF] -> Num = r#"[0-9]+"# => ActionFn(5);
            //
            fn __state5<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, Token<'input>, usize),__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Token<'input>, usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
            {
                let mut __result: (Option<(usize, Token<'input>, usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, Token(3, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action5::<>(input, __sym0);
                        let __nt = __Nonterminal::Num((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###"")""###.to_string(),
                            ]
                        });
                    }
                }
            }

            // State 6
            //     AllInputs = [r#"\\w+"#]
            //     OptionalInputs = []
            //     FixedInputs = [r#"\\w+"#]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = r#"\\w+"# (*) ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //
            //   [")", EOF] -> Term = r#"\\w+"# => ActionFn(4);
            //
            fn __state6<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, Token<'input>, usize),__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Token<'input>, usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
            {
                let mut __result: (Option<(usize, Token<'input>, usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, Token(3, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action4::<>(input, __sym0);
                        let __nt = __Nonterminal::Term((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###"")""###.to_string(),
                            ]
                        });
                    }
                }
            }

            // State 7
            //     AllInputs = ["(", Term]
            //     OptionalInputs = []
            //     FixedInputs = ["(", Term]
            //     WillPushLen = 1
            //     WillPush = [")"]
            //     WillProduce = Some(Term)
            //
            //     Term = "(" Term (*) ")" ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //
            //   ")" -> S8
            //
            fn __state7<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, Token<'input>, usize),__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __lookahead: Option<(usize, Token<'input>, usize)>,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, String, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Token<'input>, usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
            {
                let mut __result: (Option<(usize, Token<'input>, usize)>, __Nonterminal<>);
                match __lookahead {
                    Some((__loc1, Token(3, __tok0), __loc2)) => {
                        let __sym2 = (__loc1, (__tok0), __loc2);
                        __result = try!(__state8(input, __tokens, __sym0, __sym1, __sym2, ::std::marker::PhantomData::<()>));
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###"")""###.to_string(),
                            ]
                        });
                    }
                }
            }

            // State 8
            //     AllInputs = ["(", Term, ")"]
            //     OptionalInputs = []
            //     FixedInputs = ["(", Term, ")"]
            //     WillPushLen = 0
            //     WillPush = []
            //     WillProduce = Some(Term)
            //
            //     Term = "(" Term ")" (*) ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
            //
            //   [")", EOF] -> Term = "(", Term, ")" => ActionFn(2);
            //
            fn __state8<
                'input,
                __TOKENS: Iterator<Item=Result<(usize, Token<'input>, usize),__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>,
            >(
                input: &'input str,
                __tokens: &mut __TOKENS,
                __sym0: (usize, &'input str, usize),
                __sym1: (usize, String, usize),
                __sym2: (usize, &'input str, usize),
                _: ::std::marker::PhantomData<()>,
            ) -> Result<(Option<(usize, Token<'input>, usize)>, __Nonterminal<>), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
            {
                let mut __result: (Option<(usize, Token<'input>, usize)>, __Nonterminal<>);
                let __lookahead = match __tokens.next() {
                    Some(Ok(v)) => Some(v),
                    None => None,
                    Some(Err(e)) => return Err(e),
                };
                match __lookahead {
                    Some((_, Token(3, _), _)) |
                    None => {
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<>(input, __sym0, __sym1, __sym2);
                        let __nt = __Nonterminal::Term((
                            __start,
                            __nt,
                            __end,
                        ));
                        __result = (__lookahead, __nt);
                        return Ok(__result);
                    }
                    _ => {
                        return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                            token: __lookahead,
                            expected: vec![
                                r###"")""###.to_string(),
                            ]
                        });
                    }
                }
            }
        }
        pub use self::__parse__Term::TermParser;
    }
    mod __parse_table {

        mod __parse__Term {
            #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

            #[allow(unused_extern_crates)]
            extern crate lalrpop_util as __lalrpop_util;
            use super::super::super::__intern_token::Token;
            #[allow(dead_code)]
            pub enum __Symbol<'input>
             {
                Variant0(&'input str),
                Variant1(String),
            }
            const __ACTION: &'static [i8] = &[
                // State 0
                //     Num = (*) r#"[0-9]+"# ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                //     Term = (*) Num ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                //     Term = (*) "(" Term ")" ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                //     Term = (*) "22" ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                //     Term = (*) r#"\\w+"# ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                //     __Term = (*) Term ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                4,  // on "(", goto 3
                0,  // on ")", error
                5,  // on "22", goto 4
                6,  // on r#"[0-9]+"#, goto 5
                7,  // on r#"\\w+"#, goto 6

                // State 1
                //     Term = Num (*) ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                0,  // on "(", error
                -2,  // on ")", reduce `Term = Num => ActionFn(1);`
                0,  // on "22", error
                0,  // on r#"[0-9]+"#, error
                0,  // on r#"\\w+"#, error

                // State 2
                //     __Term = Term (*) ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                0,  // on "(", error
                0,  // on ")", error
                0,  // on "22", error
                0,  // on r#"[0-9]+"#, error
                0,  // on r#"\\w+"#, error

                // State 3
                //     Num = (*) r#"[0-9]+"# ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                //     Term = (*) Num ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                //     Term = (*) "(" Term ")" ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                //     Term = "(" (*) Term ")" ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                //     Term = (*) "22" ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                //     Term = (*) r#"\\w+"# ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                4,  // on "(", goto 3
                0,  // on ")", error
                5,  // on "22", goto 4
                6,  // on r#"[0-9]+"#, goto 5
                7,  // on r#"\\w+"#, goto 6

                // State 4
                //     Term = "22" (*) ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                0,  // on "(", error
                -4,  // on ")", reduce `Term = "22" => ActionFn(3);`
                0,  // on "22", error
                0,  // on r#"[0-9]+"#, error
                0,  // on r#"\\w+"#, error

                // State 5
                //     Num = r#"[0-9]+"# (*) ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                0,  // on "(", error
                -1,  // on ")", reduce `Num = r#"[0-9]+"# => ActionFn(5);`
                0,  // on "22", error
                0,  // on r#"[0-9]+"#, error
                0,  // on r#"\\w+"#, error

                // State 6
                //     Term = r#"\\w+"# (*) ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                0,  // on "(", error
                -5,  // on ")", reduce `Term = r#"\\w+"# => ActionFn(4);`
                0,  // on "22", error
                0,  // on r#"[0-9]+"#, error
                0,  // on r#"\\w+"#, error

                // State 7
                //     Term = "(" Term (*) ")" ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                0,  // on "(", error
                9,  // on ")", goto 8
                0,  // on "22", error
                0,  // on r#"[0-9]+"#, error
                0,  // on r#"\\w+"#, error

                // State 8
                //     Term = "(" Term ")" (*) ["(", ")", "22", r#"[0-9]+"#, r#"\\w+"#, EOF]
                0,  // on "(", error
                -3,  // on ")", reduce `Term = "(", Term, ")" => ActionFn(2);`
                0,  // on "22", error
                0,  // on r#"[0-9]+"#, error
                0,  // on r#"\\w+"#, error

            ];
            const __EOF_ACTION: &'static [i8] = &[
                // State 0
                0,  // on EOF, error

                // State 1
                -2,  // on EOF, reduce `Term = Num => ActionFn(1);`

                // State 2
                -6,  // on EOF, reduce `__Term = Term => ActionFn(0);`

                // State 3
                0,  // on EOF, error

                // State 4
                -4,  // on EOF, reduce `Term = "22" => ActionFn(3);`

                // State 5
                -1,  // on EOF, reduce `Num = r#"[0-9]+"# => ActionFn(5);`

                // State 6
                -5,  // on EOF, reduce `Term = r#"\\w+"# => ActionFn(4);`

                // State 7
                0,  // on EOF, error

                // State 8
                -3,  // on EOF, reduce `Term = "(", Term, ")" => ActionFn(2);`

            ];
            const __GOTO: &'static [i8] = &[
                // State 0
                2,  // on Num, goto 1
                3,  // on Term, goto 2
                0,  // on __Term, error

                // State 1
                0,  // on Num, error
                0,  // on Term, error
                0,  // on __Term, error

                // State 2
                0,  // on Num, error
                0,  // on Term, error
                0,  // on __Term, error

                // State 3
                2,  // on Num, goto 1
                8,  // on Term, goto 7
                0,  // on __Term, error

                // State 4
                0,  // on Num, error
                0,  // on Term, error
                0,  // on __Term, error

                // State 5
                0,  // on Num, error
                0,  // on Term, error
                0,  // on __Term, error

                // State 6
                0,  // on Num, error
                0,  // on Term, error
                0,  // on __Term, error

                // State 7
                0,  // on Num, error
                0,  // on Term, error
                0,  // on __Term, error

                // State 8
                0,  // on Num, error
                0,  // on Term, error
                0,  // on __Term, error

            ];
            fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
                const __TERMINAL: &'static [&'static str] = &[
                    r###""(""###,
                    r###"")""###,
                    r###""22""###,
                    r###"r#"[0-9]+"#"###,
                    r###"r#"\\w+"#"###,
                ];
                __ACTION[(__state * 5)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
                    if state == 0 {
                        None
                    } else {
                        Some(terminal.to_string())
                    }
                }).collect()
            }
            pub struct TermParser {
                builder: super::super::super::__intern_token::__MatcherBuilder,
                _priv: (),
            }

            impl TermParser {
                pub fn new() -> TermParser {
                    let __builder = super::super::super::__intern_token::__MatcherBuilder::new();
                    TermParser {
                        builder: __builder,
                        _priv: (),
                    }
                }

                #[allow(dead_code)]
                pub fn parse<
                    'input,
                >(
                    &self,
                    input: &'input str,
                ) -> Result<String, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
                {
                    let mut __tokens = self.builder.matcher(input);
                    let mut __states = vec![0_i8];
                    let mut __symbols = vec![];
                    let mut __integer;
                    let mut __lookahead;
                    let __last_location = &mut Default::default();
                    '__shift: loop {
                        __lookahead = match __tokens.next() {
                            Some(Ok(v)) => v,
                            None => break '__shift,
                            Some(Err(e)) => return Err(e),
                        };
                        *__last_location = __lookahead.2.clone();
                        __integer = match __lookahead.1 {
                            Token(2, _) if true => 0,
                            Token(3, _) if true => 1,
                            Token(4, _) if true => 2,
                            Token(1, _) if true => 3,
                            Token(0, _) if true => 4,
                            _ => {
                                let __state = *__states.last().unwrap() as usize;
                                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                                    token: Some(__lookahead),
                                    expected: __expected_tokens(__state),
                                };
                                return Err(__error);
                            }
                        };
                        '__inner: loop {
                            let __state = *__states.last().unwrap() as usize;
                            let __action = __ACTION[__state * 5 + __integer];
                            if __action > 0 {
                                let __symbol = match __integer {
                                    0 => match __lookahead.1 {
                                        Token(2, __tok0) => __Symbol::Variant0((__tok0)),
                                        _ => unreachable!(),
                                    },
                                    1 => match __lookahead.1 {
                                        Token(3, __tok0) => __Symbol::Variant0((__tok0)),
                                        _ => unreachable!(),
                                    },
                                    2 => match __lookahead.1 {
                                        Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                                        _ => unreachable!(),
                                    },
                                    3 => match __lookahead.1 {
                                        Token(1, __tok0) => __Symbol::Variant0((__tok0)),
                                        _ => unreachable!(),
                                    },
                                    4 => match __lookahead.1 {
                                        Token(0, __tok0) => __Symbol::Variant0((__tok0)),
                                        _ => unreachable!(),
                                    },
                                    _ => unreachable!(),
                                };
                                __states.push(__action - 1);
                                __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                                continue '__shift;
                            } else if __action < 0 {
                                if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                                    if r.is_err() {
                                        return r;
                                    }
                                    return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                                }
                            } else {
                                let mut __err_lookahead = Some(__lookahead);
                                let mut __err_integer: Option<usize> = Some(__integer);
                                let __state = *__states.last().unwrap() as usize;
                                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                                    token: __err_lookahead,
                                    expected: __expected_tokens(__state),
                                };
                                return Err(__error)
                            }
                        }
                    }
                    loop {
                        let __state = *__states.last().unwrap() as usize;
                        let __action = __EOF_ACTION[__state];
                        if __action < 0 {
                            if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                                return r;
                            }
                        } else {
                            let mut __err_lookahead = None;
                            let mut __err_integer: Option<usize> = None;
                            let __state = *__states.last().unwrap() as usize;
                            let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                                token: __err_lookahead,
                                expected: __expected_tokens(__state),
                            };
                            return Err(__error)
                        }
                    }
                }
            }
            pub(crate) fn __reduce<
                'input,
            >(
                input: &'input str,
                __action: i8,
                __lookahead_start: Option<&usize>,
                __states: &mut ::std::vec::Vec<i8>,
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
                _: ::std::marker::PhantomData<()>,
            ) -> Option<Result<String,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
            {
                let (__pop_states, __symbol, __nonterminal) = match -__action {
                    1 => {
                        // Num = r#"[0-9]+"# => ActionFn(5);
                        let __sym0 = __pop_Variant0(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action5::<>(input, __sym0);
                        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                        (1, __symbol, 0)
                    }
                    2 => {
                        // Term = Num => ActionFn(1);
                        let __sym0 = __pop_Variant1(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action1::<>(input, __sym0);
                        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                        (1, __symbol, 1)
                    }
                    3 => {
                        // Term = "(", Term, ")" => ActionFn(2);
                        let __sym2 = __pop_Variant0(__symbols);
                        let __sym1 = __pop_Variant1(__symbols);
                        let __sym0 = __pop_Variant0(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym2.2.clone();
                        let __nt = super::super::super::__action2::<>(input, __sym0, __sym1, __sym2);
                        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                        (3, __symbol, 1)
                    }
                    4 => {
                        // Term = "22" => ActionFn(3);
                        let __sym0 = __pop_Variant0(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action3::<>(input, __sym0);
                        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                        (1, __symbol, 1)
                    }
                    5 => {
                        // Term = r#"\\w+"# => ActionFn(4);
                        let __sym0 = __pop_Variant0(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action4::<>(input, __sym0);
                        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                        (1, __symbol, 1)
                    }
                    6 => {
                        // __Term = Term => ActionFn(0);
                        let __sym0 = __pop_Variant1(__symbols);
                        let __start = __sym0.0.clone();
                        let __end = __sym0.2.clone();
                        let __nt = super::super::super::__action0::<>(input, __sym0);
                        return Some(Ok(__nt));
                    }
                    _ => panic!("invalid action code {}", __action)
                };
                let __states_len = __states.len();
                __states.truncate(__states_len - __pop_states);
                __symbols.push(__symbol);
                let __state = *__states.last().unwrap() as usize;
                let __next_state = __GOTO[__state * 3 + __nonterminal] - 1;
                __states.push(__next_state);
                None
            }
            fn __pop_Variant1<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, String, usize)
             {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
            fn __pop_Variant0<
              'input,
            >(
                __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
            ) -> (usize, &'input str, usize)
             {
                match __symbols.pop().unwrap() {
                    (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
                    _ => panic!("symbol type mismatch")
                }
            }
        }
        pub use self::__parse__Term::TermParser;
    }
}
pub use self::__parse__Term::TermParser;
mod __intern_token {
    #![allow(unused_imports)]
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    use std::fmt as __fmt;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Token<'input>(pub usize, pub &'input str);
    impl<'a> __fmt::Display for Token<'a> {
        fn fmt(&self, formatter: &mut __fmt::Formatter) -> Result<(), __fmt::Error> {
            __fmt::Display::fmt(self.1, formatter)
        }
    }

    pub struct __MatcherBuilder {
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl __MatcherBuilder {
        pub fn new() -> __MatcherBuilder {
            let __strs: &[&str] = &[
                "^(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࡠ-ࡪࢠ-ࢴࢶ-ࢽࣔ-ࣣ࣡-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱৼ-ৼਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-૿ଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಀ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഀ-ഃഅ-ഌഎ-ഐഒ-ൄെ-ൈൊ-ൎൔ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽᲀ-ᲈ᳐-᳔᳒-᳹ᴀ-᷹᷻-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄮㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿪ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞮꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-ꣅ꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌭-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐒰-𐓓𐓘-𐓻𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑈾-𑈾𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑐀-𑑊𑑐-𑑙𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑨀-𑨾𑩇-𑩇𑩐-𑪃𑪆-𑪙𑫀-𑫸𑰀-𑰈𑰊-𑰶𑰸-𑱀𑱐-𑱙𑱲-𑲏𑲒-𑲧𑲩-𑲶𑴀-𑴆𑴈-𑴉𑴋-𑴶𑴺-𑴺𑴼-𑴽𑴿-𑵇𑵐-𑵙𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𖿠-𖿡𗀀-𘟬𘠀-𘫲𛀀-𛄞𛅰-𛋻𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞀀-𞀆𞀈-𞀘𞀛-𞀡𞀣-𞀤𞀦-𞀪𞠀-𞣄𞣐-𞣖𞤀-𞥊𞥐-𞥙𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡𬺰-𮯠丽-𪘀󠄀-󠇯])+",
                "^(?u:[0-9])+",
                "^(?u:\\()",
                "^(?u:\\))",
                "^(?u:22)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࡠ-ࡪࢠ-ࢴࢶ-ࢽࣔ-ࣣ࣡-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱৼ-ৼਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-૿ଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಀ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഀ-ഃഅ-ഌഎ-ഐഒ-ൄെ-ൈൊ-ൎൔ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽᲀ-ᲈ᳐-᳔᳒-᳹ᴀ-᷹᷻-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄮㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿪ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞮꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-ꣅ꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌭-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐒰-𐓓𐓘-𐓻𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑈾-𑈾𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑐀-𑑊𑑐-𑑙𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑨀-𑨾𑩇-𑩇𑩐-𑪃𑪆-𑪙𑫀-𑫸𑰀-𑰈𑰊-𑰶𑰸-𑱀𑱐-𑱙𑱲-𑲏𑲒-𑲧𑲩-𑲶𑴀-𑴆𑴈-𑴉𑴋-𑴶𑴺-𑴺𑴼-𑴽𑴿-𑵇𑵐-𑵙𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𖿠-𖿡𗀀-𘟬𘠀-𘫲𛀀-𛄞𛅰-𛋻𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞀀-𞀆𞀈-𞀘𞀛-𞀡𞀣-𞀤𞀦-𞀪𞠀-𞣄𞣐-𞣖𞤀-𞥊𞥐-𞥙𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡𬺰-𮯠丽-𪘀󠄀-󠇯])+").unwrap(),
                __regex::Regex::new("^(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:22)").unwrap(),
            ];
            __MatcherBuilder { regex_set: __regex_set, regex_vec: __regex_vec }
        }
        pub fn matcher<'input, 'builder>(&'builder self, s: &'input str) -> __Matcher<'input, 'builder> {
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: &self.regex_set,
                regex_vec: &self.regex_vec,
            }
        }
    }

    pub struct __Matcher<'input, 'builder> {
        text: &'input str,
        consumed: usize,
        regex_set: &'builder __regex::RegexSet,
        regex_vec: &'builder Vec<__regex::Regex>,
    }

    impl<'input, 'builder> Iterator for __Matcher<'input, 'builder> {
        type Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize,Token<'input>,&'static str>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 5 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, Token(__index, __result), __end_offset)))
                }
            }
        }
    }
}
pub use self::__intern_token::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    format!("Twenty-two!")
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    format!("Id({})", __0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    __0.to_string()
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize),&'static str> {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        value
    }
}
