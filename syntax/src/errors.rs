use crate::{lexer::LexError, tokens::Token};
use codespan::{ByteIndex, ByteOffset, Span};
use lalrpop_util::ParseError as LpError;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    InvalidToken {
        location: ByteIndex,
    },
    UnrecognizedToken {
        token: Option<(String, Span)>,
        expected: Vec<String>,
    },
    UnrecognizedEOF {
        location: ByteIndex,
        expected: Vec<String>,
    },
    ExtraToken {
        token: (String, Span),
    },
    User {
        error: LexError,
    },
}

impl ParseError {
    pub fn span(&self) -> Option<Span> {
        match *self {
            ParseError::InvalidToken { location } => {
                Some(Span::new(location, location + ByteOffset(1)))
            },
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
}

impl<'input> From<LpError<ByteIndex, Token<'input>, LexError>> for ParseError {
    fn from(other: LpError<ByteIndex, Token<'input>, LexError>) -> ParseError {
        let transform_tok = |(l, tok, r): (ByteIndex, Token, ByteIndex)| {
            (tok.to_string(), Span::new(l, r))
        };

        match other {
            LpError::InvalidToken { location } => {
                ParseError::InvalidToken { location }
            },
            LpError::UnrecognizedToken {
                token: (start, token, end),
                expected,
            } => ParseError::UnrecognizedToken {
                token: Some((token.to_string(), Span::new(start, end))),
                expected,
            },
            LpError::ExtraToken { token } => ParseError::ExtraToken {
                token: transform_tok(token),
            },
            LpError::User { error } => ParseError::User { error },
            LpError::UnrecognizedEOF { location, expected } => {
                ParseError::UnrecognizedEOF { location, expected }
            },
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
            },
            ParseError::ExtraToken {
                token: (ref tok, _),
            } => write!(f, "Extra token, {:?}", tok,),
            ParseError::User { .. } => write!(f, "Invalid character"),
            ParseError::UnrecognizedEOF { ref expected, .. } => write!(
                f,
                "Expected {} but reached the end of input",
                expected.join(" or ")
            ),
        }
    }
}

impl Error for ParseError {}
