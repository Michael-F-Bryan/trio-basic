// auto-generated: "lalrpop 0.15.2"
// sha256: 40cf95b2f34b2da92bd74a757179123eaccf86128a9247b506fd0e46d3f8e69
use codespan::{ByteIndex, Span};
use lexer::LexError;
use ast::{Ident, Literal, FunctionCall, Expr, Label, Line, Assignment, Dim, File, 
          BinaryOp, Operator, Statement};
use tokens::Token;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Assignment {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use codespan::{ByteIndex, Span};
    use lexer::LexError;
    use ast::{Ident, Literal, FunctionCall, Expr, Label, Line, Assignment, Dim, File, 
          BinaryOp, Operator, Statement};
    use tokens::Token;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(::std::vec::Vec<Token<'input>>),
        Variant2(Expr),
        Variant3(::std::vec::Vec<Expr>),
        Variant4(ByteIndex),
        Variant5(Assignment),
        Variant6(Vec<Expr>),
        Variant7(Dim),
        Variant8(::std::option::Option<Expr>),
        Variant9(File),
        Variant10(FunctionCall),
        Variant11(Ident),
        Variant12(Label),
        Variant13(Line),
        Variant14(::std::vec::Vec<Line>),
        Variant15(Literal),
        Variant16(Statement),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        -28, -28, -28, -28, -28, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 4, 14, 0, 15,
        // State 5
        0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, -45, -45, -45, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, -22, -22, -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        17, -20, -20, -20, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, -17, 18, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 4, 14, 0, 15,
        // State 12
        0, -38, -38, -38, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, -37, -37, -37, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, -39, -39, -39, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 4, 14, 0, 15,
        // State 16
        12, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 4, 14, 0, 15,
        // State 17
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 4, 14, 0, 15,
        // State 18
        0, 25, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, -16, 18, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        12, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 4, 14, 0, 15,
        // State 21
        0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, -11, 0, 16, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, -13, 0, 16, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, -27, -27, -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        -6, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, -6, -6, 0, -6,
        // State 28
        -7, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, -7, -7, 0, -7,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -46,
        // State 2
        0,
        // State 3
        -28,
        // State 4
        0,
        // State 5
        -10,
        // State 6
        -45,
        // State 7
        -22,
        // State 8
        -20,
        // State 9
        -21,
        // State 10
        -17,
        // State 11
        0,
        // State 12
        -38,
        // State 13
        -37,
        // State 14
        -39,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        -16,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        -44,
        // State 24
        -23,
        // State 25
        0,
        // State 26
        -27,
        // State 27
        0,
        // State 28
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 7, 0, 8, 9, 0, 0, 0, 10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 7, 0, 8, 9, 0, 0, 0, 10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 8, 9, 0, 0, 0, 10, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 21, 0, 0, 0, 22, 0, 23, 0, 7, 0, 8, 9, 0, 0, 0, 10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 8, 9, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 7, 0, 8, 9, 0, 0, 0, 10, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"":""###,
            r###""=""###,
            r###""as""###,
            r###""dim""###,
            r###""eol""###,
            r###""float""###,
            r###""goto""###,
            r###""ident""###,
            r###""int""###,
            r###""print""###,
            r###""string""###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub(crate) struct AssignmentParser {
        _priv: (),
    }

    impl AssignmentParser {
        pub(crate) fn new() -> AssignmentParser {
            AssignmentParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub(crate) fn parse<
            'input,
            __TOKEN: __ToTriple<'input, Error=LexError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Assignment, __lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token::OpenParen if true => 0,
                    Token::CloseParen if true => 1,
                    Token::Asterisk if true => 2,
                    Token::Plus if true => 3,
                    Token::Comma if true => 4,
                    Token::Minus if true => 5,
                    Token::Colon if true => 6,
                    Token::Equals if true => 7,
                    Token::As if true => 8,
                    Token::Dim if true => 9,
                    Token::EndOfLine if true => 10,
                    Token::Float(_) if true => 11,
                    Token::Goto if true => 12,
                    Token::Identifier(_) if true => 13,
                    Token::Integer(_) if true => 14,
                    Token::Print if true => 15,
                    Token::String(_) if true => 16,
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
                    let __action = __ACTION[__state * 17 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ Token::OpenParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ Token::CloseParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ Token::Asterisk => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ Token::Equals => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ Token::As => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ Token::Dim => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ Token::EndOfLine => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ Token::Float(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ Token::Goto => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ Token::Identifier(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ Token::Integer(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ Token::Print => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ Token::String(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Assignment,__lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                // __Assignment = Assignment => ActionFn(3);
                let __sym0 = __pop_Variant5(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                return Some(Ok(__nt));
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Assignment, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ByteIndex, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Dim, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Expr, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, File, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionCall, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Ident, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Label, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Line, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Literal, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Statement, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Token<'input>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::option::Option<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Line>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol"+, "eol" => ActionFn(39);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",") = Expr, "," => ActionFn(48);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 1)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* =  => ActionFn(46);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action46::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(47);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = Expr, "," => ActionFn(51);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action52::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 3)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @L =  => ActionFn(43);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action43::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @R =  => ActionFn(40);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action40::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Assignment = Ident, "=", Expr => ActionFn(69);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = Expr => ActionFn(83);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action83::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> =  => ActionFn(84);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action84::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(85);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 7)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(86);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Dim = "dim", Ident, "as", Ident => ActionFn(70);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (4, __symbol, 8)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Expr, "+", Term => ActionFn(71);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action71::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Term => ActionFn(23);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? = Expr => ActionFn(44);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 10)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (0, __symbol, 10)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Ident => ActionFn(26);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Literal => ActionFn(27);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = FunctionCall => ActionFn(28);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = "(", Expr, ")" => ActionFn(29);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 11)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol", File => ActionFn(9);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 12)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = Line+ => ActionFn(72);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol" => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionCall = Ident, "(", Comma<Expr>, ")" => ActionFn(74);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (4, __symbol, 13)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident = "ident" => ActionFn(75);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Label = "ident", ":" => ActionFn(76);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 15)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Label, "eol"+ => ActionFn(12);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Assignment, "eol"+ => ActionFn(13);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action13::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Dim, "eol"+ => ActionFn(14);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = FunctionCall, "eol"+ => ActionFn(15);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Statement, "eol"+ => ActionFn(16);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action16::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line => ActionFn(41);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line+, Line => ActionFn(42);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (2, __symbol, 17)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "int" => ActionFn(77);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "float" => ActionFn(78);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "string" => ActionFn(79);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"print"> => ActionFn(17);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"goto"> => ActionFn(18);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"goto"> = "goto", Comma<Expr> => ActionFn(80);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action80::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"print"> = "print", Comma<Expr> => ActionFn(81);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action81::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 21)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Term, "*", Factor => ActionFn(82);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Factor => ActionFn(25);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Dim = Dim => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Expr = Expr => ActionFn(5);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __File = File => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __FunctionCall = FunctionCall => ActionFn(8);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Ident = Ident => ActionFn(7);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Label = Label => ActionFn(2);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Line = Line => ActionFn(1);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Literal = Literal => ActionFn(6);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 31)
    }
}
pub(crate) use self::__parse__Assignment::AssignmentParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Dim {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use codespan::{ByteIndex, Span};
    use lexer::LexError;
    use ast::{Ident, Literal, FunctionCall, Expr, Label, Line, Assignment, Dim, File, 
          BinaryOp, Operator, Statement};
    use tokens::Token;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(::std::vec::Vec<Token<'input>>),
        Variant2(Expr),
        Variant3(::std::vec::Vec<Expr>),
        Variant4(ByteIndex),
        Variant5(Assignment),
        Variant6(Vec<Expr>),
        Variant7(Dim),
        Variant8(::std::option::Option<Expr>),
        Variant9(File),
        Variant10(FunctionCall),
        Variant11(Ident),
        Variant12(Label),
        Variant13(Line),
        Variant14(::std::vec::Vec<Line>),
        Variant15(Literal),
        Variant16(Statement),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -47,
        // State 2
        0,
        // State 3
        0,
        // State 4
        -28,
        // State 5
        0,
        // State 6
        -15,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"":""###,
            r###""=""###,
            r###""as""###,
            r###""dim""###,
            r###""eol""###,
            r###""float""###,
            r###""goto""###,
            r###""ident""###,
            r###""int""###,
            r###""print""###,
            r###""string""###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub(crate) struct DimParser {
        _priv: (),
    }

    impl DimParser {
        pub(crate) fn new() -> DimParser {
            DimParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub(crate) fn parse<
            'input,
            __TOKEN: __ToTriple<'input, Error=LexError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Dim, __lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token::OpenParen if true => 0,
                    Token::CloseParen if true => 1,
                    Token::Asterisk if true => 2,
                    Token::Plus if true => 3,
                    Token::Comma if true => 4,
                    Token::Minus if true => 5,
                    Token::Colon if true => 6,
                    Token::Equals if true => 7,
                    Token::As if true => 8,
                    Token::Dim if true => 9,
                    Token::EndOfLine if true => 10,
                    Token::Float(_) if true => 11,
                    Token::Goto if true => 12,
                    Token::Identifier(_) if true => 13,
                    Token::Integer(_) if true => 14,
                    Token::Print if true => 15,
                    Token::String(_) if true => 16,
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
                    let __action = __ACTION[__state * 17 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ Token::OpenParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ Token::CloseParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ Token::Asterisk => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ Token::Equals => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ Token::As => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ Token::Dim => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ Token::EndOfLine => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ Token::Float(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ Token::Goto => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ Token::Identifier(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ Token::Integer(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ Token::Print => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ Token::String(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Dim,__lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                // __Dim = Dim => ActionFn(4);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                return Some(Ok(__nt));
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Assignment, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ByteIndex, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Dim, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Expr, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, File, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionCall, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Ident, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Label, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Line, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Literal, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Statement, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Token<'input>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::option::Option<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Line>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol"+, "eol" => ActionFn(39);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",") = Expr, "," => ActionFn(48);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 1)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* =  => ActionFn(46);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action46::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(47);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = Expr, "," => ActionFn(51);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action52::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 3)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @L =  => ActionFn(43);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action43::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @R =  => ActionFn(40);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action40::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Assignment = Ident, "=", Expr => ActionFn(69);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = Expr => ActionFn(83);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action83::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> =  => ActionFn(84);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action84::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(85);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 7)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(86);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Dim = "dim", Ident, "as", Ident => ActionFn(70);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (4, __symbol, 8)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Expr, "+", Term => ActionFn(71);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action71::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Term => ActionFn(23);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? = Expr => ActionFn(44);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 10)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (0, __symbol, 10)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Ident => ActionFn(26);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Literal => ActionFn(27);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = FunctionCall => ActionFn(28);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = "(", Expr, ")" => ActionFn(29);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 11)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol", File => ActionFn(9);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 12)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = Line+ => ActionFn(72);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol" => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionCall = Ident, "(", Comma<Expr>, ")" => ActionFn(74);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (4, __symbol, 13)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident = "ident" => ActionFn(75);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Label = "ident", ":" => ActionFn(76);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 15)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Label, "eol"+ => ActionFn(12);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Assignment, "eol"+ => ActionFn(13);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action13::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Dim, "eol"+ => ActionFn(14);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = FunctionCall, "eol"+ => ActionFn(15);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Statement, "eol"+ => ActionFn(16);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action16::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line => ActionFn(41);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line+, Line => ActionFn(42);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (2, __symbol, 17)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "int" => ActionFn(77);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "float" => ActionFn(78);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "string" => ActionFn(79);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"print"> => ActionFn(17);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"goto"> => ActionFn(18);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"goto"> = "goto", Comma<Expr> => ActionFn(80);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action80::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"print"> = "print", Comma<Expr> => ActionFn(81);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action81::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 21)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Term, "*", Factor => ActionFn(82);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Factor => ActionFn(25);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Assignment = Assignment => ActionFn(3);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Expr = Expr => ActionFn(5);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __File = File => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __FunctionCall = FunctionCall => ActionFn(8);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Ident = Ident => ActionFn(7);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Label = Label => ActionFn(2);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Line = Line => ActionFn(1);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Literal = Literal => ActionFn(6);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 31)
    }
}
pub(crate) use self::__parse__Dim::DimParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use codespan::{ByteIndex, Span};
    use lexer::LexError;
    use ast::{Ident, Literal, FunctionCall, Expr, Label, Line, Assignment, Dim, File, 
          BinaryOp, Operator, Statement};
    use tokens::Token;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(::std::vec::Vec<Token<'input>>),
        Variant2(Expr),
        Variant3(::std::vec::Vec<Expr>),
        Variant4(ByteIndex),
        Variant5(Assignment),
        Variant6(Vec<Expr>),
        Variant7(Dim),
        Variant8(::std::option::Option<Expr>),
        Variant9(File),
        Variant10(FunctionCall),
        Variant11(Ident),
        Variant12(Label),
        Variant13(Line),
        Variant14(::std::vec::Vec<Line>),
        Variant15(Literal),
        Variant16(Statement),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 10, 11, 0, 12,
        // State 1
        0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, -45, -45, -45, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, -22, -22, -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        14, -20, -20, -20, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, -17, 15, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 10, 11, 0, 12,
        // State 8
        0, -38, -38, -38, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, -37, -37, -37, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, -39, -39, -39, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 10, 11, 0, 12,
        // State 13
        8, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 10, 11, 0, 12,
        // State 14
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 10, 11, 0, 12,
        // State 15
        0, 22, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, -16, 15, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        8, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 10, 11, 0, 12,
        // State 18
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, -11, 0, 13, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, -13, 0, 13, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, -27, -27, -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        -6, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, -6, -6, 0, -6,
        // State 25
        -7, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, -7, -7, 0, -7,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -48,
        // State 2
        -45,
        // State 3
        -22,
        // State 4
        -20,
        // State 5
        -21,
        // State 6
        -17,
        // State 7
        0,
        // State 8
        -38,
        // State 9
        -28,
        // State 10
        -37,
        // State 11
        -39,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        -16,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        -44,
        // State 21
        -23,
        // State 22
        0,
        // State 23
        -27,
        // State 24
        0,
        // State 25
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 4, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 3, 0, 4, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 4, 5, 0, 0, 0, 6, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 18, 0, 0, 0, 19, 0, 20, 0, 3, 0, 4, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 4, 5, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 3, 0, 4, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"":""###,
            r###""=""###,
            r###""as""###,
            r###""dim""###,
            r###""eol""###,
            r###""float""###,
            r###""goto""###,
            r###""ident""###,
            r###""int""###,
            r###""print""###,
            r###""string""###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct ExprParser {
        _priv: (),
    }

    impl ExprParser {
        pub fn new() -> ExprParser {
            ExprParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            __TOKEN: __ToTriple<'input, Error=LexError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Expr, __lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token::OpenParen if true => 0,
                    Token::CloseParen if true => 1,
                    Token::Asterisk if true => 2,
                    Token::Plus if true => 3,
                    Token::Comma if true => 4,
                    Token::Minus if true => 5,
                    Token::Colon if true => 6,
                    Token::Equals if true => 7,
                    Token::As if true => 8,
                    Token::Dim if true => 9,
                    Token::EndOfLine if true => 10,
                    Token::Float(_) if true => 11,
                    Token::Goto if true => 12,
                    Token::Identifier(_) if true => 13,
                    Token::Integer(_) if true => 14,
                    Token::Print if true => 15,
                    Token::String(_) if true => 16,
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
                    let __action = __ACTION[__state * 17 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ Token::OpenParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ Token::CloseParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ Token::Asterisk => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ Token::Equals => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ Token::As => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ Token::Dim => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ Token::EndOfLine => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ Token::Float(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ Token::Goto => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ Token::Identifier(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ Token::Integer(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ Token::Print => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ Token::String(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Expr,__lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                // __Expr = Expr => ActionFn(5);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                return Some(Ok(__nt));
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Assignment, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ByteIndex, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Dim, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Expr, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, File, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionCall, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Ident, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Label, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Line, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Literal, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Statement, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Token<'input>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::option::Option<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Line>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol"+, "eol" => ActionFn(39);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",") = Expr, "," => ActionFn(48);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 1)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* =  => ActionFn(46);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action46::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(47);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = Expr, "," => ActionFn(51);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action52::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 3)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @L =  => ActionFn(43);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action43::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @R =  => ActionFn(40);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action40::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Assignment = Ident, "=", Expr => ActionFn(69);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = Expr => ActionFn(83);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action83::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> =  => ActionFn(84);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action84::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(85);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 7)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(86);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Dim = "dim", Ident, "as", Ident => ActionFn(70);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (4, __symbol, 8)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Expr, "+", Term => ActionFn(71);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action71::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Term => ActionFn(23);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? = Expr => ActionFn(44);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 10)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (0, __symbol, 10)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Ident => ActionFn(26);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Literal => ActionFn(27);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = FunctionCall => ActionFn(28);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = "(", Expr, ")" => ActionFn(29);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 11)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol", File => ActionFn(9);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 12)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = Line+ => ActionFn(72);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol" => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionCall = Ident, "(", Comma<Expr>, ")" => ActionFn(74);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (4, __symbol, 13)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident = "ident" => ActionFn(75);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Label = "ident", ":" => ActionFn(76);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 15)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Label, "eol"+ => ActionFn(12);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Assignment, "eol"+ => ActionFn(13);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action13::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Dim, "eol"+ => ActionFn(14);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = FunctionCall, "eol"+ => ActionFn(15);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Statement, "eol"+ => ActionFn(16);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action16::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line => ActionFn(41);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line+, Line => ActionFn(42);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (2, __symbol, 17)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "int" => ActionFn(77);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "float" => ActionFn(78);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "string" => ActionFn(79);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"print"> => ActionFn(17);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"goto"> => ActionFn(18);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"goto"> = "goto", Comma<Expr> => ActionFn(80);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action80::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"print"> = "print", Comma<Expr> => ActionFn(81);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action81::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 21)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Term, "*", Factor => ActionFn(82);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Factor => ActionFn(25);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Assignment = Assignment => ActionFn(3);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Dim = Dim => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __File = File => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __FunctionCall = FunctionCall => ActionFn(8);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Ident = Ident => ActionFn(7);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Label = Label => ActionFn(2);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Line = Line => ActionFn(1);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Literal = Literal => ActionFn(6);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 31)
    }
}
pub use self::__parse__Expr::ExprParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__File {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use codespan::{ByteIndex, Span};
    use lexer::LexError;
    use ast::{Ident, Literal, FunctionCall, Expr, Label, Line, Assignment, Dim, File, 
          BinaryOp, Operator, Statement};
    use tokens::Token;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(::std::vec::Vec<Token<'input>>),
        Variant2(Expr),
        Variant3(::std::vec::Vec<Expr>),
        Variant4(ByteIndex),
        Variant5(Assignment),
        Variant6(Vec<Expr>),
        Variant7(Dim),
        Variant8(::std::option::Option<Expr>),
        Variant9(File),
        Variant10(FunctionCall),
        Variant11(Ident),
        Variant12(Label),
        Variant13(Line),
        Variant14(::std::vec::Vec<Line>),
        Variant15(Literal),
        Variant16(Statement),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14, 0, 15, 16, 0, 17, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0,
        // State 5
        22, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, -35, -35, 0, -35, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 15, 16, 0, 17, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14, 0, 15, 16, 0, 17, 0,
        // State 14
        38, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 39, 0, 28, 40, 0, 41,
        // State 15
        -28, 0, 0, 0, 0, 0, 42, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        38, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 39, 0, 28, 40, 0, 41,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 44, 0, -31, -31, 0, -31, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 0, -1, -1, 0, -1, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 44, 0, -32, -32, 0, -32, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 44, 0, -33, -33, 0, -33, 0,
        // State 21
        38, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 28, 40, 0, 41,
        // State 22
        38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 28, 40, 0, 41,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 44, 0, -30, -30, 0, -30, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, -36, -36, 0, -36, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, -34, 44, 0, -34, -34, 0, -34, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        -28, -28, -28, -28, -28, 0, 0, 0, -28, 0, -28, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        38, -14, 0, 0, 0, 0, 0, 0, 0, 0, -14, 39, 0, 28, 40, 0, 41,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -11, 0, 49, 50, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0,
        // State 32
        0, -45, -45, -45, -45, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0,
        // State 33
        0, -22, -22, -22, -22, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0,
        // State 34
        22, -20, -20, -20, -20, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0,
        // State 35
        0, -21, -21, -21, -21, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0,
        // State 36
        0, -17, 51, -17, -17, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0,
        // State 37
        38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 28, 40, 0, 41,
        // State 38
        0, -38, -38, -38, -38, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0,
        // State 39
        0, -37, -37, -37, -37, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0,
        // State 40
        0, -39, -39, -39, -39, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, -2, -2, 0, -2, -2, 0, -2, 0,
        // State 44
        0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 49, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0,
        // State 47
        0, -13, 0, 49, 55, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0,
        // State 48
        38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 28, 40, 0, 41,
        // State 49
        -6, -6, 0, 0, 0, 0, 0, 0, 0, 0, -6, -6, 0, -6, -6, 0, -6,
        // State 50
        38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 28, 40, 0, 41,
        // State 51
        0, 58, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, -27, -27, -27, -27, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0,
        // State 54
        -7, -7, 0, 0, 0, 0, 0, 0, 0, 0, -7, -7, 0, -7, -7, 0, -7,
        // State 55
        0, -16, 51, -16, -16, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0,
        // State 56
        0, -44, -44, -44, -44, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0,
        // State 57
        0, -23, -23, -23, -23, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        -49,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        -35,
        // State 8
        -25,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        -26,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        -31,
        // State 18
        -1,
        // State 19
        -32,
        // State 20
        -33,
        // State 21
        0,
        // State 22
        0,
        // State 23
        -30,
        // State 24
        -36,
        // State 25
        -34,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -24,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        -2,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 4, 5, 6, 7, 8, 9, 0, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 0, 5, 6, 7, 25, 0, 0, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 29, 5, 6, 7, 8, 9, 0, 10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 30, 0, 0, 0, 31, 0, 32, 0, 33, 0, 34, 35, 0, 0, 0, 36, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 30, 0, 0, 0, 43, 0, 32, 0, 33, 0, 34, 35, 0, 0, 0, 36, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 30, 0, 0, 0, 45, 0, 32, 0, 33, 0, 34, 35, 0, 0, 0, 36, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 33, 0, 34, 35, 0, 0, 0, 36, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 33, 0, 34, 35, 0, 0, 0, 36, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 33, 0, 34, 35, 0, 0, 0, 36, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 34, 35, 0, 0, 0, 36, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 34, 35, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"":""###,
            r###""=""###,
            r###""as""###,
            r###""dim""###,
            r###""eol""###,
            r###""float""###,
            r###""goto""###,
            r###""ident""###,
            r###""int""###,
            r###""print""###,
            r###""string""###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct FileParser {
        _priv: (),
    }

    impl FileParser {
        pub fn new() -> FileParser {
            FileParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            __TOKEN: __ToTriple<'input, Error=LexError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<File, __lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token::OpenParen if true => 0,
                    Token::CloseParen if true => 1,
                    Token::Asterisk if true => 2,
                    Token::Plus if true => 3,
                    Token::Comma if true => 4,
                    Token::Minus if true => 5,
                    Token::Colon if true => 6,
                    Token::Equals if true => 7,
                    Token::As if true => 8,
                    Token::Dim if true => 9,
                    Token::EndOfLine if true => 10,
                    Token::Float(_) if true => 11,
                    Token::Goto if true => 12,
                    Token::Identifier(_) if true => 13,
                    Token::Integer(_) if true => 14,
                    Token::Print if true => 15,
                    Token::String(_) if true => 16,
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
                    let __action = __ACTION[__state * 17 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ Token::OpenParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ Token::CloseParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ Token::Asterisk => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ Token::Equals => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ Token::As => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ Token::Dim => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ Token::EndOfLine => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ Token::Float(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ Token::Goto => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ Token::Identifier(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ Token::Integer(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ Token::Print => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ Token::String(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<File,__lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                // __File = File => ActionFn(0);
                let __sym0 = __pop_Variant9(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Assignment, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ByteIndex, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Dim, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Expr, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, File, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionCall, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Ident, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Label, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Line, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Literal, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Statement, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Token<'input>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::option::Option<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Line>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol"+, "eol" => ActionFn(39);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",") = Expr, "," => ActionFn(48);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 1)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* =  => ActionFn(46);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action46::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(47);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = Expr, "," => ActionFn(51);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action52::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 3)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @L =  => ActionFn(43);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action43::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @R =  => ActionFn(40);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action40::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Assignment = Ident, "=", Expr => ActionFn(69);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = Expr => ActionFn(83);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action83::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> =  => ActionFn(84);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action84::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(85);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 7)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(86);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Dim = "dim", Ident, "as", Ident => ActionFn(70);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (4, __symbol, 8)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Expr, "+", Term => ActionFn(71);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action71::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Term => ActionFn(23);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? = Expr => ActionFn(44);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 10)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (0, __symbol, 10)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Ident => ActionFn(26);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Literal => ActionFn(27);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = FunctionCall => ActionFn(28);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = "(", Expr, ")" => ActionFn(29);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 11)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol", File => ActionFn(9);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 12)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = Line+ => ActionFn(72);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol" => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionCall = Ident, "(", Comma<Expr>, ")" => ActionFn(74);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (4, __symbol, 13)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident = "ident" => ActionFn(75);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Label = "ident", ":" => ActionFn(76);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 15)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Label, "eol"+ => ActionFn(12);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Assignment, "eol"+ => ActionFn(13);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action13::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Dim, "eol"+ => ActionFn(14);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = FunctionCall, "eol"+ => ActionFn(15);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Statement, "eol"+ => ActionFn(16);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action16::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line => ActionFn(41);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line+, Line => ActionFn(42);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (2, __symbol, 17)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "int" => ActionFn(77);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "float" => ActionFn(78);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "string" => ActionFn(79);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"print"> => ActionFn(17);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"goto"> => ActionFn(18);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"goto"> = "goto", Comma<Expr> => ActionFn(80);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action80::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"print"> = "print", Comma<Expr> => ActionFn(81);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action81::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 21)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Term, "*", Factor => ActionFn(82);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Factor => ActionFn(25);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Assignment = Assignment => ActionFn(3);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Dim = Dim => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Expr = Expr => ActionFn(5);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __FunctionCall = FunctionCall => ActionFn(8);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Ident = Ident => ActionFn(7);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Label = Label => ActionFn(2);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Line = Line => ActionFn(1);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Literal = Literal => ActionFn(6);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 31)
    }
}
pub use self::__parse__File::FileParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__FunctionCall {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use codespan::{ByteIndex, Span};
    use lexer::LexError;
    use ast::{Ident, Literal, FunctionCall, Expr, Label, Line, Assignment, Dim, File, 
          BinaryOp, Operator, Statement};
    use tokens::Token;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(::std::vec::Vec<Token<'input>>),
        Variant2(Expr),
        Variant3(::std::vec::Vec<Expr>),
        Variant4(ByteIndex),
        Variant5(Assignment),
        Variant6(Vec<Expr>),
        Variant7(Dim),
        Variant8(::std::option::Option<Expr>),
        Variant9(File),
        Variant10(FunctionCall),
        Variant11(Ident),
        Variant12(Label),
        Variant13(Line),
        Variant14(::std::vec::Vec<Line>),
        Variant15(Literal),
        Variant16(Statement),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        14, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 4, 16, 0, 17,
        // State 5
        14, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 4, 16, 0, 17,
        // State 6
        0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, -11, 0, 20, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, -45, -45, -45, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, -22, -22, -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        5, -20, -20, -20, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, -17, 22, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 4, 16, 0, 17,
        // State 14
        0, -38, -38, -38, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, -37, -37, -37, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, -39, -39, -39, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, -13, 0, 20, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, -27, -27, -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 4, 16, 0, 17,
        // State 20
        -6, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, -6, -6, 0, -6,
        // State 21
        14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 4, 16, 0, 17,
        // State 22
        0, 27, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        -7, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, -7, -7, 0, -7,
        // State 24
        0, -16, 22, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -50,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        -27,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 6, 0, 0, 0, 7, 0, 8, 0, 9, 0, 10, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 9, 0, 10, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 9, 0, 10, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 10, 11, 0, 0, 0, 12, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 10, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"":""###,
            r###""=""###,
            r###""as""###,
            r###""dim""###,
            r###""eol""###,
            r###""float""###,
            r###""goto""###,
            r###""ident""###,
            r###""int""###,
            r###""print""###,
            r###""string""###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub(crate) struct FunctionCallParser {
        _priv: (),
    }

    impl FunctionCallParser {
        pub(crate) fn new() -> FunctionCallParser {
            FunctionCallParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub(crate) fn parse<
            'input,
            __TOKEN: __ToTriple<'input, Error=LexError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<FunctionCall, __lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token::OpenParen if true => 0,
                    Token::CloseParen if true => 1,
                    Token::Asterisk if true => 2,
                    Token::Plus if true => 3,
                    Token::Comma if true => 4,
                    Token::Minus if true => 5,
                    Token::Colon if true => 6,
                    Token::Equals if true => 7,
                    Token::As if true => 8,
                    Token::Dim if true => 9,
                    Token::EndOfLine if true => 10,
                    Token::Float(_) if true => 11,
                    Token::Goto if true => 12,
                    Token::Identifier(_) if true => 13,
                    Token::Integer(_) if true => 14,
                    Token::Print if true => 15,
                    Token::String(_) if true => 16,
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
                    let __action = __ACTION[__state * 17 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ Token::OpenParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ Token::CloseParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ Token::Asterisk => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ Token::Equals => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ Token::As => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ Token::Dim => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ Token::EndOfLine => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ Token::Float(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ Token::Goto => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ Token::Identifier(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ Token::Integer(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ Token::Print => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ Token::String(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<FunctionCall,__lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                // __FunctionCall = FunctionCall => ActionFn(8);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(__sym0);
                return Some(Ok(__nt));
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Assignment, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ByteIndex, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Dim, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Expr, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, File, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionCall, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Ident, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Label, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Line, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Literal, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Statement, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Token<'input>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::option::Option<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Line>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol"+, "eol" => ActionFn(39);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",") = Expr, "," => ActionFn(48);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 1)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* =  => ActionFn(46);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action46::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(47);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = Expr, "," => ActionFn(51);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action52::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 3)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @L =  => ActionFn(43);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action43::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @R =  => ActionFn(40);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action40::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Assignment = Ident, "=", Expr => ActionFn(69);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = Expr => ActionFn(83);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action83::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> =  => ActionFn(84);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action84::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(85);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 7)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(86);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Dim = "dim", Ident, "as", Ident => ActionFn(70);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (4, __symbol, 8)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Expr, "+", Term => ActionFn(71);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action71::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Term => ActionFn(23);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? = Expr => ActionFn(44);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 10)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (0, __symbol, 10)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Ident => ActionFn(26);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Literal => ActionFn(27);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = FunctionCall => ActionFn(28);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = "(", Expr, ")" => ActionFn(29);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 11)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol", File => ActionFn(9);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 12)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = Line+ => ActionFn(72);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol" => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionCall = Ident, "(", Comma<Expr>, ")" => ActionFn(74);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (4, __symbol, 13)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident = "ident" => ActionFn(75);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Label = "ident", ":" => ActionFn(76);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 15)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Label, "eol"+ => ActionFn(12);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Assignment, "eol"+ => ActionFn(13);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action13::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Dim, "eol"+ => ActionFn(14);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = FunctionCall, "eol"+ => ActionFn(15);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Statement, "eol"+ => ActionFn(16);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action16::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line => ActionFn(41);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line+, Line => ActionFn(42);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (2, __symbol, 17)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "int" => ActionFn(77);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "float" => ActionFn(78);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "string" => ActionFn(79);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"print"> => ActionFn(17);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"goto"> => ActionFn(18);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"goto"> = "goto", Comma<Expr> => ActionFn(80);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action80::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"print"> = "print", Comma<Expr> => ActionFn(81);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action81::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 21)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Term, "*", Factor => ActionFn(82);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Factor => ActionFn(25);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Assignment = Assignment => ActionFn(3);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Dim = Dim => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Expr = Expr => ActionFn(5);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __File = File => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Ident = Ident => ActionFn(7);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Label = Label => ActionFn(2);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Line = Line => ActionFn(1);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Literal = Literal => ActionFn(6);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 31)
    }
}
pub(crate) use self::__parse__FunctionCall::FunctionCallParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Ident {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use codespan::{ByteIndex, Span};
    use lexer::LexError;
    use ast::{Ident, Literal, FunctionCall, Expr, Label, Line, Assignment, Dim, File, 
          BinaryOp, Operator, Statement};
    use tokens::Token;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(::std::vec::Vec<Token<'input>>),
        Variant2(Expr),
        Variant3(::std::vec::Vec<Expr>),
        Variant4(ByteIndex),
        Variant5(Assignment),
        Variant6(Vec<Expr>),
        Variant7(Dim),
        Variant8(::std::option::Option<Expr>),
        Variant9(File),
        Variant10(FunctionCall),
        Variant11(Ident),
        Variant12(Label),
        Variant13(Line),
        Variant14(::std::vec::Vec<Line>),
        Variant15(Literal),
        Variant16(Statement),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -51,
        // State 2
        -28,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"":""###,
            r###""=""###,
            r###""as""###,
            r###""dim""###,
            r###""eol""###,
            r###""float""###,
            r###""goto""###,
            r###""ident""###,
            r###""int""###,
            r###""print""###,
            r###""string""###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub(crate) struct IdentParser {
        _priv: (),
    }

    impl IdentParser {
        pub(crate) fn new() -> IdentParser {
            IdentParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub(crate) fn parse<
            'input,
            __TOKEN: __ToTriple<'input, Error=LexError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Ident, __lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token::OpenParen if true => 0,
                    Token::CloseParen if true => 1,
                    Token::Asterisk if true => 2,
                    Token::Plus if true => 3,
                    Token::Comma if true => 4,
                    Token::Minus if true => 5,
                    Token::Colon if true => 6,
                    Token::Equals if true => 7,
                    Token::As if true => 8,
                    Token::Dim if true => 9,
                    Token::EndOfLine if true => 10,
                    Token::Float(_) if true => 11,
                    Token::Goto if true => 12,
                    Token::Identifier(_) if true => 13,
                    Token::Integer(_) if true => 14,
                    Token::Print if true => 15,
                    Token::String(_) if true => 16,
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
                    let __action = __ACTION[__state * 17 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ Token::OpenParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ Token::CloseParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ Token::Asterisk => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ Token::Equals => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ Token::As => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ Token::Dim => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ Token::EndOfLine => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ Token::Float(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ Token::Goto => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ Token::Identifier(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ Token::Integer(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ Token::Print => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ Token::String(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Ident,__lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                // __Ident = Ident => ActionFn(7);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(__sym0);
                return Some(Ok(__nt));
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Assignment, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ByteIndex, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Dim, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Expr, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, File, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionCall, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Ident, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Label, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Line, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Literal, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Statement, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Token<'input>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::option::Option<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Line>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol"+, "eol" => ActionFn(39);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",") = Expr, "," => ActionFn(48);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 1)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* =  => ActionFn(46);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action46::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(47);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = Expr, "," => ActionFn(51);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action52::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 3)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @L =  => ActionFn(43);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action43::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @R =  => ActionFn(40);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action40::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Assignment = Ident, "=", Expr => ActionFn(69);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = Expr => ActionFn(83);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action83::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> =  => ActionFn(84);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action84::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(85);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 7)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(86);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Dim = "dim", Ident, "as", Ident => ActionFn(70);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (4, __symbol, 8)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Expr, "+", Term => ActionFn(71);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action71::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Term => ActionFn(23);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? = Expr => ActionFn(44);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 10)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (0, __symbol, 10)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Ident => ActionFn(26);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Literal => ActionFn(27);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = FunctionCall => ActionFn(28);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = "(", Expr, ")" => ActionFn(29);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 11)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol", File => ActionFn(9);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 12)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = Line+ => ActionFn(72);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol" => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionCall = Ident, "(", Comma<Expr>, ")" => ActionFn(74);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (4, __symbol, 13)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident = "ident" => ActionFn(75);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Label = "ident", ":" => ActionFn(76);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 15)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Label, "eol"+ => ActionFn(12);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Assignment, "eol"+ => ActionFn(13);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action13::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Dim, "eol"+ => ActionFn(14);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = FunctionCall, "eol"+ => ActionFn(15);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Statement, "eol"+ => ActionFn(16);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action16::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line => ActionFn(41);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line+, Line => ActionFn(42);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (2, __symbol, 17)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "int" => ActionFn(77);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "float" => ActionFn(78);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "string" => ActionFn(79);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"print"> => ActionFn(17);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"goto"> => ActionFn(18);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"goto"> = "goto", Comma<Expr> => ActionFn(80);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action80::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"print"> = "print", Comma<Expr> => ActionFn(81);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action81::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 21)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Term, "*", Factor => ActionFn(82);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Factor => ActionFn(25);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Assignment = Assignment => ActionFn(3);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Dim = Dim => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Expr = Expr => ActionFn(5);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __File = File => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __FunctionCall = FunctionCall => ActionFn(8);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Label = Label => ActionFn(2);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Line = Line => ActionFn(1);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Literal = Literal => ActionFn(6);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 31)
    }
}
pub(crate) use self::__parse__Ident::IdentParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Label {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use codespan::{ByteIndex, Span};
    use lexer::LexError;
    use ast::{Ident, Literal, FunctionCall, Expr, Label, Line, Assignment, Dim, File, 
          BinaryOp, Operator, Statement};
    use tokens::Token;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(::std::vec::Vec<Token<'input>>),
        Variant2(Expr),
        Variant3(::std::vec::Vec<Expr>),
        Variant4(ByteIndex),
        Variant5(Assignment),
        Variant6(Vec<Expr>),
        Variant7(Dim),
        Variant8(::std::option::Option<Expr>),
        Variant9(File),
        Variant10(FunctionCall),
        Variant11(Ident),
        Variant12(Label),
        Variant13(Line),
        Variant14(::std::vec::Vec<Line>),
        Variant15(Literal),
        Variant16(Statement),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -52,
        // State 2
        0,
        // State 3
        -29,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"":""###,
            r###""=""###,
            r###""as""###,
            r###""dim""###,
            r###""eol""###,
            r###""float""###,
            r###""goto""###,
            r###""ident""###,
            r###""int""###,
            r###""print""###,
            r###""string""###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub(crate) struct LabelParser {
        _priv: (),
    }

    impl LabelParser {
        pub(crate) fn new() -> LabelParser {
            LabelParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub(crate) fn parse<
            'input,
            __TOKEN: __ToTriple<'input, Error=LexError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Label, __lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token::OpenParen if true => 0,
                    Token::CloseParen if true => 1,
                    Token::Asterisk if true => 2,
                    Token::Plus if true => 3,
                    Token::Comma if true => 4,
                    Token::Minus if true => 5,
                    Token::Colon if true => 6,
                    Token::Equals if true => 7,
                    Token::As if true => 8,
                    Token::Dim if true => 9,
                    Token::EndOfLine if true => 10,
                    Token::Float(_) if true => 11,
                    Token::Goto if true => 12,
                    Token::Identifier(_) if true => 13,
                    Token::Integer(_) if true => 14,
                    Token::Print if true => 15,
                    Token::String(_) if true => 16,
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
                    let __action = __ACTION[__state * 17 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ Token::OpenParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ Token::CloseParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ Token::Asterisk => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ Token::Equals => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ Token::As => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ Token::Dim => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ Token::EndOfLine => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ Token::Float(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ Token::Goto => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ Token::Identifier(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ Token::Integer(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ Token::Print => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ Token::String(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Label,__lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                // __Label = Label => ActionFn(2);
                let __sym0 = __pop_Variant12(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                return Some(Ok(__nt));
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Assignment, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ByteIndex, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Dim, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Expr, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, File, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionCall, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Ident, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Label, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Line, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Literal, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Statement, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Token<'input>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::option::Option<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Line>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol"+, "eol" => ActionFn(39);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",") = Expr, "," => ActionFn(48);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 1)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* =  => ActionFn(46);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action46::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(47);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = Expr, "," => ActionFn(51);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action52::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 3)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @L =  => ActionFn(43);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action43::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @R =  => ActionFn(40);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action40::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Assignment = Ident, "=", Expr => ActionFn(69);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = Expr => ActionFn(83);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action83::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> =  => ActionFn(84);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action84::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(85);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 7)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(86);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Dim = "dim", Ident, "as", Ident => ActionFn(70);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (4, __symbol, 8)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Expr, "+", Term => ActionFn(71);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action71::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Term => ActionFn(23);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? = Expr => ActionFn(44);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 10)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (0, __symbol, 10)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Ident => ActionFn(26);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Literal => ActionFn(27);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = FunctionCall => ActionFn(28);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = "(", Expr, ")" => ActionFn(29);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 11)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol", File => ActionFn(9);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 12)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = Line+ => ActionFn(72);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol" => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionCall = Ident, "(", Comma<Expr>, ")" => ActionFn(74);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (4, __symbol, 13)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident = "ident" => ActionFn(75);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Label = "ident", ":" => ActionFn(76);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 15)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Label, "eol"+ => ActionFn(12);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Assignment, "eol"+ => ActionFn(13);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action13::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Dim, "eol"+ => ActionFn(14);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = FunctionCall, "eol"+ => ActionFn(15);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Statement, "eol"+ => ActionFn(16);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action16::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line => ActionFn(41);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line+, Line => ActionFn(42);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (2, __symbol, 17)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "int" => ActionFn(77);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "float" => ActionFn(78);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "string" => ActionFn(79);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"print"> => ActionFn(17);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"goto"> => ActionFn(18);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"goto"> = "goto", Comma<Expr> => ActionFn(80);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action80::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"print"> = "print", Comma<Expr> => ActionFn(81);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action81::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 21)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Term, "*", Factor => ActionFn(82);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Factor => ActionFn(25);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Assignment = Assignment => ActionFn(3);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Dim = Dim => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Expr = Expr => ActionFn(5);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __File = File => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __FunctionCall = FunctionCall => ActionFn(8);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Ident = Ident => ActionFn(7);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Line = Line => ActionFn(1);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Literal = Literal => ActionFn(6);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 31)
    }
}
pub(crate) use self::__parse__Label::LabelParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Line {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use codespan::{ByteIndex, Span};
    use lexer::LexError;
    use ast::{Ident, Literal, FunctionCall, Expr, Label, Line, Assignment, Dim, File, 
          BinaryOp, Operator, Statement};
    use tokens::Token;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(::std::vec::Vec<Token<'input>>),
        Variant2(Expr),
        Variant3(::std::vec::Vec<Expr>),
        Variant4(ByteIndex),
        Variant5(Assignment),
        Variant6(Vec<Expr>),
        Variant7(Dim),
        Variant8(::std::option::Option<Expr>),
        Variant9(File),
        Variant10(FunctionCall),
        Variant11(Ident),
        Variant12(Label),
        Variant13(Line),
        Variant14(::std::vec::Vec<Line>),
        Variant15(Literal),
        Variant16(Statement),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 12, 13, 0, 14, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0,
        // State 4
        19, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0,
        // State 11
        33, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 34, 0, 24, 35, 0, 36,
        // State 12
        -28, 0, 0, 0, 0, 0, 37, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        33, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 34, 0, 24, 35, 0, 36,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0,
        // State 18
        33, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0, 24, 35, 0, 36,
        // State 19
        33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0, 24, 35, 0, 36,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        -28, -28, -28, -28, -28, 0, 0, 0, -28, 0, -28, 0, 0, 0, 0, 0, 0,
        // State 24
        33, -14, 0, 0, 0, 0, 0, 0, 0, 0, -14, 34, 0, 24, 35, 0, 36,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0,
        // State 26
        0, -11, 0, 44, 45, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0,
        // State 27
        0, -45, -45, -45, -45, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0,
        // State 28
        0, -22, -22, -22, -22, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0,
        // State 29
        19, -20, -20, -20, -20, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0,
        // State 30
        0, -21, -21, -21, -21, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -17, 46, -17, -17, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0,
        // State 32
        33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0, 24, 35, 0, 36,
        // State 33
        0, -38, -38, -38, -38, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0,
        // State 34
        0, -37, -37, -37, -37, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0,
        // State 35
        0, -39, -39, -39, -39, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 44, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0,
        // State 42
        0, -13, 0, 44, 50, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0,
        // State 43
        33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0, 24, 35, 0, 36,
        // State 44
        -6, -6, 0, 0, 0, 0, 0, 0, 0, 0, -6, -6, 0, -6, -6, 0, -6,
        // State 45
        33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0, 24, 35, 0, 36,
        // State 46
        0, 53, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, -27, -27, -27, -27, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0,
        // State 49
        -7, -7, 0, 0, 0, 0, 0, 0, 0, 0, -7, -7, 0, -7, -7, 0, -7,
        // State 50
        0, -16, 46, -16, -16, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0,
        // State 51
        0, -44, -44, -44, -44, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0,
        // State 52
        0, -23, -23, -23, -23, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        -53,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        -31,
        // State 15
        -1,
        // State 16
        -32,
        // State 17
        -33,
        // State 18
        0,
        // State 19
        0,
        // State 20
        -30,
        // State 21
        -34,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        -2,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 0, 4, 5, 6, 7, 0, 0, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 25, 0, 0, 0, 26, 0, 27, 0, 28, 0, 29, 30, 0, 0, 0, 31, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 25, 0, 0, 0, 38, 0, 27, 0, 28, 0, 29, 30, 0, 0, 0, 31, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 25, 0, 0, 0, 40, 0, 27, 0, 28, 0, 29, 30, 0, 0, 0, 31, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 28, 0, 29, 30, 0, 0, 0, 31, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 28, 0, 29, 30, 0, 0, 0, 31, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 28, 0, 29, 30, 0, 0, 0, 31, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 29, 30, 0, 0, 0, 31, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 29, 30, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"":""###,
            r###""=""###,
            r###""as""###,
            r###""dim""###,
            r###""eol""###,
            r###""float""###,
            r###""goto""###,
            r###""ident""###,
            r###""int""###,
            r###""print""###,
            r###""string""###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct LineParser {
        _priv: (),
    }

    impl LineParser {
        pub fn new() -> LineParser {
            LineParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            __TOKEN: __ToTriple<'input, Error=LexError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Line, __lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token::OpenParen if true => 0,
                    Token::CloseParen if true => 1,
                    Token::Asterisk if true => 2,
                    Token::Plus if true => 3,
                    Token::Comma if true => 4,
                    Token::Minus if true => 5,
                    Token::Colon if true => 6,
                    Token::Equals if true => 7,
                    Token::As if true => 8,
                    Token::Dim if true => 9,
                    Token::EndOfLine if true => 10,
                    Token::Float(_) if true => 11,
                    Token::Goto if true => 12,
                    Token::Identifier(_) if true => 13,
                    Token::Integer(_) if true => 14,
                    Token::Print if true => 15,
                    Token::String(_) if true => 16,
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
                    let __action = __ACTION[__state * 17 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ Token::OpenParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ Token::CloseParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ Token::Asterisk => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ Token::Equals => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ Token::As => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ Token::Dim => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ Token::EndOfLine => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ Token::Float(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ Token::Goto => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ Token::Identifier(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ Token::Integer(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ Token::Print => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ Token::String(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Line,__lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                // __Line = Line => ActionFn(1);
                let __sym0 = __pop_Variant13(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                return Some(Ok(__nt));
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Assignment, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ByteIndex, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Dim, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Expr, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, File, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionCall, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Ident, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Label, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Line, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Literal, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Statement, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Token<'input>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::option::Option<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Line>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol"+, "eol" => ActionFn(39);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",") = Expr, "," => ActionFn(48);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 1)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* =  => ActionFn(46);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action46::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(47);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = Expr, "," => ActionFn(51);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action52::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 3)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @L =  => ActionFn(43);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action43::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @R =  => ActionFn(40);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action40::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Assignment = Ident, "=", Expr => ActionFn(69);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = Expr => ActionFn(83);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action83::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> =  => ActionFn(84);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action84::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(85);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 7)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(86);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Dim = "dim", Ident, "as", Ident => ActionFn(70);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (4, __symbol, 8)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Expr, "+", Term => ActionFn(71);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action71::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Term => ActionFn(23);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? = Expr => ActionFn(44);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 10)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (0, __symbol, 10)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Ident => ActionFn(26);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Literal => ActionFn(27);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = FunctionCall => ActionFn(28);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = "(", Expr, ")" => ActionFn(29);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 11)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol", File => ActionFn(9);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 12)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = Line+ => ActionFn(72);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol" => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionCall = Ident, "(", Comma<Expr>, ")" => ActionFn(74);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (4, __symbol, 13)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident = "ident" => ActionFn(75);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Label = "ident", ":" => ActionFn(76);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 15)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Label, "eol"+ => ActionFn(12);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Assignment, "eol"+ => ActionFn(13);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action13::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Dim, "eol"+ => ActionFn(14);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = FunctionCall, "eol"+ => ActionFn(15);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Statement, "eol"+ => ActionFn(16);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action16::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line => ActionFn(41);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line+, Line => ActionFn(42);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (2, __symbol, 17)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "int" => ActionFn(77);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "float" => ActionFn(78);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "string" => ActionFn(79);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"print"> => ActionFn(17);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"goto"> => ActionFn(18);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"goto"> = "goto", Comma<Expr> => ActionFn(80);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action80::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"print"> = "print", Comma<Expr> => ActionFn(81);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action81::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 21)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Term, "*", Factor => ActionFn(82);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Factor => ActionFn(25);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Assignment = Assignment => ActionFn(3);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Dim = Dim => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Expr = Expr => ActionFn(5);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __File = File => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __FunctionCall = FunctionCall => ActionFn(8);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Ident = Ident => ActionFn(7);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Label = Label => ActionFn(2);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce54<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Literal = Literal => ActionFn(6);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 31)
    }
}
pub use self::__parse__Line::LineParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Literal {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use codespan::{ByteIndex, Span};
    use lexer::LexError;
    use ast::{Ident, Literal, FunctionCall, Expr, Label, Line, Assignment, Dim, File, 
          BinaryOp, Operator, Statement};
    use tokens::Token;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(::std::vec::Vec<Token<'input>>),
        Variant2(Expr),
        Variant3(::std::vec::Vec<Expr>),
        Variant4(ByteIndex),
        Variant5(Assignment),
        Variant6(Vec<Expr>),
        Variant7(Dim),
        Variant8(::std::option::Option<Expr>),
        Variant9(File),
        Variant10(FunctionCall),
        Variant11(Ident),
        Variant12(Label),
        Variant13(Line),
        Variant14(::std::vec::Vec<Line>),
        Variant15(Literal),
        Variant16(Statement),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 4, 0, 5,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -54,
        // State 2
        -38,
        // State 3
        -37,
        // State 4
        -39,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"":""###,
            r###""=""###,
            r###""as""###,
            r###""dim""###,
            r###""eol""###,
            r###""float""###,
            r###""goto""###,
            r###""ident""###,
            r###""int""###,
            r###""print""###,
            r###""string""###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct LiteralParser {
        _priv: (),
    }

    impl LiteralParser {
        pub fn new() -> LiteralParser {
            LiteralParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            __TOKEN: __ToTriple<'input, Error=LexError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Literal, __lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token::OpenParen if true => 0,
                    Token::CloseParen if true => 1,
                    Token::Asterisk if true => 2,
                    Token::Plus if true => 3,
                    Token::Comma if true => 4,
                    Token::Minus if true => 5,
                    Token::Colon if true => 6,
                    Token::Equals if true => 7,
                    Token::As if true => 8,
                    Token::Dim if true => 9,
                    Token::EndOfLine if true => 10,
                    Token::Float(_) if true => 11,
                    Token::Goto if true => 12,
                    Token::Identifier(_) if true => 13,
                    Token::Integer(_) if true => 14,
                    Token::Print if true => 15,
                    Token::String(_) if true => 16,
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
                    let __action = __ACTION[__state * 17 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ Token::OpenParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ Token::CloseParen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ Token::Asterisk => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ Token::Equals => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ Token::As => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ Token::Dim => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ Token::EndOfLine => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ Token::Float(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ Token::Goto => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ Token::Identifier(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ Token::Integer(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ Token::Print => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ Token::String(_) => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Literal,__lalrpop_util::ParseError<ByteIndex, Token<'input>, LexError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                // __Literal = Literal => ActionFn(6);
                let __sym0 = __pop_Variant15(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Assignment, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ByteIndex, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Dim, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Expr, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, File, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, FunctionCall, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Ident, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Label, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Line, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Literal, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Statement, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Token<'input>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::option::Option<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Line>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>
    ) -> (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // "eol"+ = "eol"+, "eol" => ActionFn(39);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action39::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",") = Expr, "," => ActionFn(48);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 1)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* =  => ActionFn(46);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action46::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")* = (<Expr> ",")+ => ActionFn(47);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = Expr, "," => ActionFn(51);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action51::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // (<Expr> ",")+ = (<Expr> ",")+, Expr, "," => ActionFn(52);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action52::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 3)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @L =  => ActionFn(43);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action43::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // @R =  => ActionFn(40);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action40::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Assignment = Ident, "=", Expr => ActionFn(69);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action69::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 6)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = Expr => ActionFn(83);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action83::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> =  => ActionFn(84);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action84::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+, Expr => ActionFn(85);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action85::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 7)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Comma<Expr> = (<Expr> ",")+ => ActionFn(86);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action86::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Dim = "dim", Ident, "as", Ident => ActionFn(70);
        let __sym3 = __pop_Variant11(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant11(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action70::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (4, __symbol, 8)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Expr, "+", Term => ActionFn(71);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action71::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr = Term => ActionFn(23);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? = Expr => ActionFn(44);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action44::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 10)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Expr? =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (0, __symbol, 10)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Ident => ActionFn(26);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = Literal => ActionFn(27);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = FunctionCall => ActionFn(28);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Factor = "(", Expr, ")" => ActionFn(29);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 11)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol", File => ActionFn(9);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 12)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = Line+ => ActionFn(72);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // File = "eol" => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // FunctionCall = Ident, "(", Comma<Expr>, ")" => ActionFn(74);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (4, __symbol, 13)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Ident = "ident" => ActionFn(75);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Label = "ident", ":" => ActionFn(76);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action76::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 15)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Label, "eol"+ => ActionFn(12);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Assignment, "eol"+ => ActionFn(13);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action13::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Dim, "eol"+ => ActionFn(14);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = FunctionCall, "eol"+ => ActionFn(15);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line = Statement, "eol"+ => ActionFn(16);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action16::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (2, __symbol, 16)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line => ActionFn(41);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Line+ = Line+, Line => ActionFn(42);
        let __sym1 = __pop_Variant13(__symbols);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (2, __symbol, 17)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "int" => ActionFn(77);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "float" => ActionFn(78);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Literal = "string" => ActionFn(79);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action79::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"print"> => ActionFn(17);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Statement = Stmt<"goto"> => ActionFn(18);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"goto"> = "goto", Comma<Expr> => ActionFn(80);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action80::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Stmt<"print"> = "print", Comma<Expr> => ActionFn(81);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action81::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (2, __symbol, 21)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Term, "*", Factor => ActionFn(82);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action82::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // Term = Factor => ActionFn(25);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Assignment = Assignment => ActionFn(3);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Dim = Dim => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Expr = Expr => ActionFn(5);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __File = File => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __FunctionCall = FunctionCall => ActionFn(8);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Ident = Ident => ActionFn(7);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce52<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Label = Label => ActionFn(2);
        let __sym0 = __pop_Variant12(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce53<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&ByteIndex>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(ByteIndex,__Symbol<'input>,ByteIndex)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (ByteIndex,__Symbol<'input>,ByteIndex), usize)
    {
        // __Line = Line => ActionFn(1);
        let __sym0 = __pop_Variant13(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 30)
    }
}
pub use self::__parse__Literal::LiteralParser;

fn __action0<
    'input,
>(
    (_, __0, _): (ByteIndex, File, ByteIndex),
) -> File
{
    (__0)
}

fn __action1<
    'input,
>(
    (_, __0, _): (ByteIndex, Line, ByteIndex),
) -> Line
{
    (__0)
}

fn __action2<
    'input,
>(
    (_, __0, _): (ByteIndex, Label, ByteIndex),
) -> Label
{
    (__0)
}

fn __action3<
    'input,
>(
    (_, __0, _): (ByteIndex, Assignment, ByteIndex),
) -> Assignment
{
    (__0)
}

fn __action4<
    'input,
>(
    (_, __0, _): (ByteIndex, Dim, ByteIndex),
) -> Dim
{
    (__0)
}

fn __action5<
    'input,
>(
    (_, __0, _): (ByteIndex, Expr, ByteIndex),
) -> Expr
{
    (__0)
}

fn __action6<
    'input,
>(
    (_, __0, _): (ByteIndex, Literal, ByteIndex),
) -> Literal
{
    (__0)
}

fn __action7<
    'input,
>(
    (_, __0, _): (ByteIndex, Ident, ByteIndex),
) -> Ident
{
    (__0)
}

fn __action8<
    'input,
>(
    (_, __0, _): (ByteIndex, FunctionCall, ByteIndex),
) -> FunctionCall
{
    (__0)
}

fn __action9<
    'input,
>(
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, __0, _): (ByteIndex, File, ByteIndex),
) -> File
{
    __0
}

fn __action10<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, lines, _): (ByteIndex, ::std::vec::Vec<Line>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> File
{
    File::new(lines, Span::new(l, r))
}

fn __action11<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> File
{
    File::new(Vec::new(), Span::new(l, r))
}

fn __action12<
    'input,
>(
    (_, l, _): (ByteIndex, Label, ByteIndex),
    (_, _, _): (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex),
) -> Line
{
    l.into()
}

fn __action13<
    'input,
>(
    (_, a, _): (ByteIndex, Assignment, ByteIndex),
    (_, _, _): (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex),
) -> Line
{
    a.into()
}

fn __action14<
    'input,
>(
    (_, d, _): (ByteIndex, Dim, ByteIndex),
    (_, _, _): (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex),
) -> Line
{
    d.into()
}

fn __action15<
    'input,
>(
    (_, f, _): (ByteIndex, FunctionCall, ByteIndex),
    (_, _, _): (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex),
) -> Line
{
    f.into()
}

fn __action16<
    'input,
>(
    (_, s, _): (ByteIndex, Statement, ByteIndex),
    (_, _, _): (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex),
) -> Line
{
    s.into()
}

fn __action17<
    'input,
>(
    (_, __0, _): (ByteIndex, Statement, ByteIndex),
) -> Statement
{
    (__0)
}

fn __action18<
    'input,
>(
    (_, __0, _): (ByteIndex, Statement, ByteIndex),
) -> Statement
{
    (__0)
}

fn __action19<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, n, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Label
{
    Label::new(n.as_ident().unwrap(), Span::new(l, r))
}

fn __action20<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, n, _): (ByteIndex, Ident, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, v, _): (ByteIndex, Expr, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Assignment
{
    Assignment::new(n, v, Span::new(l, r))
}

fn __action21<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, n, _): (ByteIndex, Ident, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, t, _): (ByteIndex, Ident, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Dim
{
    Dim::new(n, t, Span::new(l, r))
}

fn __action22<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, left, _): (ByteIndex, Expr, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, right, _): (ByteIndex, Expr, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Expr
{
    Expr::BinaryOp(BinaryOp::new(left, right, Operator::Add, Span::new(l, r)))
}

fn __action23<
    'input,
>(
    (_, __0, _): (ByteIndex, Expr, ByteIndex),
) -> Expr
{
    __0
}

fn __action24<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, left, _): (ByteIndex, Expr, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, right, _): (ByteIndex, Expr, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Expr
{
    Expr::BinaryOp(BinaryOp::new(left, right, Operator::Multiply, Span::new(l, r)))
}

fn __action25<
    'input,
>(
    (_, __0, _): (ByteIndex, Expr, ByteIndex),
) -> Expr
{
    __0
}

fn __action26<
    'input,
>(
    (_, __0, _): (ByteIndex, Ident, ByteIndex),
) -> Expr
{
    Expr::Ident(__0)
}

fn __action27<
    'input,
>(
    (_, __0, _): (ByteIndex, Literal, ByteIndex),
) -> Expr
{
    Expr::Literal(__0)
}

fn __action28<
    'input,
>(
    (_, __0, _): (ByteIndex, FunctionCall, ByteIndex),
) -> Expr
{
    Expr::FunctionCall(__0)
}

fn __action29<
    'input,
>(
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, __0, _): (ByteIndex, Expr, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
) -> Expr
{
    __0
}

fn __action30<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, i, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Literal
{
    Literal::new(i.as_int().unwrap(), Span::new(l, r))
}

fn __action31<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, i, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Literal
{
    Literal::new(i.as_float().unwrap(), Span::new(l, r))
}

fn __action32<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, i, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Literal
{
    Literal::new(i.as_str().unwrap(), Span::new(l, r))
}

fn __action33<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, id, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Ident
{
    Ident::new(id.as_ident().unwrap(), Span::new(l, r))
}

fn __action34<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, name, _): (ByteIndex, Ident, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, args, _): (ByteIndex, Vec<Expr>, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> FunctionCall
{
    FunctionCall::new(name, args, Span::new(l, r))
}

fn __action35<
    'input,
>(
    (_, v, _): (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex),
    (_, e, _): (ByteIndex, ::std::option::Option<Expr>, ByteIndex),
) -> Vec<Expr>
{
    match e { 
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

fn __action36<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, p, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, args, _): (ByteIndex, Vec<Expr>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Statement
{
    Statement::new(p.as_str().unwrap(), args, Span::new(l, r))
}

fn __action37<
    'input,
>(
    (_, l, _): (ByteIndex, ByteIndex, ByteIndex),
    (_, p, _): (ByteIndex, Token<'input>, ByteIndex),
    (_, args, _): (ByteIndex, Vec<Expr>, ByteIndex),
    (_, r, _): (ByteIndex, ByteIndex, ByteIndex),
) -> Statement
{
    Statement::new(p.as_str().unwrap(), args, Span::new(l, r))
}

fn __action38<
    'input,
>(
    (_, __0, _): (ByteIndex, Token<'input>, ByteIndex),
) -> ::std::vec::Vec<Token<'input>>
{
    vec![__0]
}

fn __action39<
    'input,
>(
    (_, v, _): (ByteIndex, ::std::vec::Vec<Token<'input>>, ByteIndex),
    (_, e, _): (ByteIndex, Token<'input>, ByteIndex),
) -> ::std::vec::Vec<Token<'input>>
{
    { let mut v = v; v.push(e); v }
}

fn __action40<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> ByteIndex
{
    __lookbehind.clone()
}

fn __action41<
    'input,
>(
    (_, __0, _): (ByteIndex, Line, ByteIndex),
) -> ::std::vec::Vec<Line>
{
    vec![__0]
}

fn __action42<
    'input,
>(
    (_, v, _): (ByteIndex, ::std::vec::Vec<Line>, ByteIndex),
    (_, e, _): (ByteIndex, Line, ByteIndex),
) -> ::std::vec::Vec<Line>
{
    { let mut v = v; v.push(e); v }
}

fn __action43<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> ByteIndex
{
    __lookahead.clone()
}

fn __action44<
    'input,
>(
    (_, __0, _): (ByteIndex, Expr, ByteIndex),
) -> ::std::option::Option<Expr>
{
    Some(__0)
}

fn __action45<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> ::std::option::Option<Expr>
{
    None
}

fn __action46<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> ::std::vec::Vec<Expr>
{
    vec![]
}

fn __action47<
    'input,
>(
    (_, v, _): (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex),
) -> ::std::vec::Vec<Expr>
{
    v
}

fn __action48<
    'input,
>(
    (_, __0, _): (ByteIndex, Expr, ByteIndex),
    (_, _, _): (ByteIndex, Token<'input>, ByteIndex),
) -> Expr
{
    (__0)
}

fn __action49<
    'input,
>(
    (_, __0, _): (ByteIndex, Expr, ByteIndex),
) -> ::std::vec::Vec<Expr>
{
    vec![__0]
}

fn __action50<
    'input,
>(
    (_, v, _): (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex),
    (_, e, _): (ByteIndex, Expr, ByteIndex),
) -> ::std::vec::Vec<Expr>
{
    { let mut v = v; v.push(e); v }
}

fn __action51<
    'input,
>(
    __0: (ByteIndex, Expr, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
) -> ::std::vec::Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action48(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        __temp0,
    )
}

fn __action52<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex),
    __1: (ByteIndex, Expr, ByteIndex),
    __2: (ByteIndex, Token<'input>, ByteIndex),
) -> ::std::vec::Vec<Expr>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action48(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        __0,
        __temp0,
    )
}

fn __action53<
    'input,
>(
    __0: (ByteIndex, ::std::option::Option<Expr>, ByteIndex),
) -> Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action46(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        __temp0,
        __0,
    )
}

fn __action54<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex),
    __1: (ByteIndex, ::std::option::Option<Expr>, ByteIndex),
) -> Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        __temp0,
        __1,
    )
}

fn __action55<
    'input,
>(
    __0: (ByteIndex, Ident, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, Expr, ByteIndex),
    __3: (ByteIndex, ByteIndex, ByteIndex),
) -> Assignment
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

fn __action56<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, Ident, ByteIndex),
    __2: (ByteIndex, Token<'input>, ByteIndex),
    __3: (ByteIndex, Ident, ByteIndex),
    __4: (ByteIndex, ByteIndex, ByteIndex),
) -> Dim
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

fn __action57<
    'input,
>(
    __0: (ByteIndex, Expr, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, Expr, ByteIndex),
    __3: (ByteIndex, ByteIndex, ByteIndex),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

fn __action58<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Line>, ByteIndex),
    __1: (ByteIndex, ByteIndex, ByteIndex),
) -> File
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        __temp0,
        __0,
        __1,
    )
}

fn __action59<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, ByteIndex, ByteIndex),
) -> File
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        __temp0,
        __0,
        __1,
    )
}

fn __action60<
    'input,
>(
    __0: (ByteIndex, Ident, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, Vec<Expr>, ByteIndex),
    __3: (ByteIndex, Token<'input>, ByteIndex),
    __4: (ByteIndex, ByteIndex, ByteIndex),
) -> FunctionCall
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        __temp0,
        __0,
        __1,
        __2,
        __3,
        __4,
    )
}

fn __action61<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, ByteIndex, ByteIndex),
) -> Ident
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        __temp0,
        __0,
        __1,
    )
}

fn __action62<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, ByteIndex, ByteIndex),
) -> Label
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        __temp0,
        __0,
        __1,
        __2,
    )
}

fn __action63<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, ByteIndex, ByteIndex),
) -> Literal
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        __temp0,
        __0,
        __1,
    )
}

fn __action64<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, ByteIndex, ByteIndex),
) -> Literal
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        __temp0,
        __0,
        __1,
    )
}

fn __action65<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, ByteIndex, ByteIndex),
) -> Literal
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action32(
        __temp0,
        __0,
        __1,
    )
}

fn __action66<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, Vec<Expr>, ByteIndex),
    __2: (ByteIndex, ByteIndex, ByteIndex),
) -> Statement
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        __temp0,
        __0,
        __1,
        __2,
    )
}

fn __action67<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, Vec<Expr>, ByteIndex),
    __2: (ByteIndex, ByteIndex, ByteIndex),
) -> Statement
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        __temp0,
        __0,
        __1,
        __2,
    )
}

fn __action68<
    'input,
>(
    __0: (ByteIndex, Expr, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, Expr, ByteIndex),
    __3: (ByteIndex, ByteIndex, ByteIndex),
) -> Expr
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action43(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

fn __action69<
    'input,
>(
    __0: (ByteIndex, Ident, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, Expr, ByteIndex),
) -> Assignment
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        __0,
        __1,
        __2,
        __temp0,
    )
}

fn __action70<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, Ident, ByteIndex),
    __2: (ByteIndex, Token<'input>, ByteIndex),
    __3: (ByteIndex, Ident, ByteIndex),
) -> Dim
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

fn __action71<
    'input,
>(
    __0: (ByteIndex, Expr, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, Expr, ByteIndex),
) -> Expr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        __0,
        __1,
        __2,
        __temp0,
    )
}

fn __action72<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Line>, ByteIndex),
) -> File
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        __0,
        __temp0,
    )
}

fn __action73<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
) -> File
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        __0,
        __temp0,
    )
}

fn __action74<
    'input,
>(
    __0: (ByteIndex, Ident, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, Vec<Expr>, ByteIndex),
    __3: (ByteIndex, Token<'input>, ByteIndex),
) -> FunctionCall
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

fn __action75<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
) -> Ident
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        __0,
        __temp0,
    )
}

fn __action76<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
) -> Label
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        __0,
        __1,
        __temp0,
    )
}

fn __action77<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
) -> Literal
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        __0,
        __temp0,
    )
}

fn __action78<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
) -> Literal
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        __0,
        __temp0,
    )
}

fn __action79<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
) -> Literal
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        __0,
        __temp0,
    )
}

fn __action80<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, Vec<Expr>, ByteIndex),
) -> Statement
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action66(
        __0,
        __1,
        __temp0,
    )
}

fn __action81<
    'input,
>(
    __0: (ByteIndex, Token<'input>, ByteIndex),
    __1: (ByteIndex, Vec<Expr>, ByteIndex),
) -> Statement
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action67(
        __0,
        __1,
        __temp0,
    )
}

fn __action82<
    'input,
>(
    __0: (ByteIndex, Expr, ByteIndex),
    __1: (ByteIndex, Token<'input>, ByteIndex),
    __2: (ByteIndex, Expr, ByteIndex),
) -> Expr
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action68(
        __0,
        __1,
        __2,
        __temp0,
    )
}

fn __action83<
    'input,
>(
    __0: (ByteIndex, Expr, ByteIndex),
) -> Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        __temp0,
    )
}

fn __action84<
    'input,
>(
    __lookbehind: &ByteIndex,
    __lookahead: &ByteIndex,
) -> Vec<Expr>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action45(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        __temp0,
    )
}

fn __action85<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex),
    __1: (ByteIndex, Expr, ByteIndex),
) -> Vec<Expr>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action44(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        __0,
        __temp0,
    )
}

fn __action86<
    'input,
>(
    __0: (ByteIndex, ::std::vec::Vec<Expr>, ByteIndex),
) -> Vec<Expr>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action45(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(ByteIndex,Token<'input>,ByteIndex),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (ByteIndex, Token<'input>, ByteIndex) {
    type Error = LexError;
    fn to_triple(value: Self) -> Result<(ByteIndex,Token<'input>,ByteIndex),LexError> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(ByteIndex, Token<'input>, ByteIndex),LexError> {
    type Error = LexError;
    fn to_triple(value: Self) -> Result<(ByteIndex,Token<'input>,ByteIndex),LexError> {
        value
    }
}
