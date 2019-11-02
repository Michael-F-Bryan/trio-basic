use codespan::{ByteIndex, ByteOffset, Span};
use lalrpop_util::ParseError as LpError;
use lexer::LexError;
use std::fmt::{self, Display, Formatter};
use tokens::Token;

#[derive(Debug, Clone, PartialEq, Fail)]
pub enum ParseError {
    InvalidToken {
        location: ByteIndex,
    },
    UnrecognizedToken {
        token: Option<(String, Span<ByteIndex>)>,
        expected: Vec<String>,
    },
    ExtraToken {
        token: (String, Span<ByteIndex>),
    },
    User {
        error: LexError,
    },
}

impl ParseError {
    pub fn span(&self) -> Option<Span<ByteIndex>> {
        match *self {
            ParseError::InvalidToken { location } => {
                Some(Span::new(location, location + ByteOffset(1)))
            }
            ParseError::User {
                error: LexError { span },
            }
            | ParseError::ExtraToken { token: (_, span) }
            | ParseError::UnrecognizedToken {
                token: Some((_, span)),
                ..
            } => Some(span),
            _ => None,
        }
    }

    /// Returns the location of the offending token or character. If the error
    /// occurred at the end of the file, `None` is returned.
    pub fn location(&self) -> Option<ByteIndex> {
        match *self {
            ParseError::InvalidToken { location } => Some(location),
            ParseError::User {
                error: LexError { span },
            }
            | ParseError::ExtraToken { token: (_, span) }
            | ParseError::UnrecognizedToken {
                token: Some((_, span)),
                ..
            } => Some(span.start()),
            _ => None,
        }
    }

    /// Update any location information attached to this `ParseError` by adding
    /// the specified offset.
    pub fn offset_inplace(&mut self, offset: ByteOffset) {
        match *self {
            ParseError::InvalidToken { ref mut location } => *location += offset,
            ParseError::User {
                error: LexError { ref mut span },
            }
            | ParseError::ExtraToken {
                token: (_, ref mut span),
            }
            | ParseError::UnrecognizedToken {
                token: Some((_, ref mut span)),
                ..
            } => *span = span.map(|ix| ix + offset),
            _ => {}
        }
    }
}

impl<'input> From<LpError<ByteIndex, Token<'input>, LexError>> for ParseError {
    fn from(other: LpError<ByteIndex, Token<'input>, LexError>) -> ParseError {
        let transform_tok =
            |(l, tok, r): (ByteIndex, Token, ByteIndex)| (tok.to_string(), Span::new(l, r));

        match other {
            LpError::InvalidToken { location } => ParseError::InvalidToken { location },
            LpError::UnrecognizedToken { token, expected } => ParseError::UnrecognizedToken {
                token: token.map(transform_tok),
                expected,
            },
            LpError::ExtraToken { token } => ParseError::ExtraToken {
                token: transform_tok(token),
            },
            LpError::User { error } => ParseError::User { error },
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ParseError::InvalidToken { .. } => write!(f, "Invalid token"),
            ParseError::UnrecognizedToken {
                ref token,
                ref expected,
            } => {
                let expected = expected.join(" or ");
                write!(f, "Expected {} but ", expected)?;

                match token {
                    Some((ref got, _)) => write!(f, "found \"{}\"", got),
                    None => write!(f, "reached the end of input"),
                }
            }
            ParseError::ExtraToken {
                token: (ref tok, _),
            } => write!(f, "Extra token, {:?}", tok,),
            ParseError::User { .. } => write!(f, "Invalid character"),
        }
    }
}
