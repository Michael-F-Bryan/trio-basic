use codespan::ByteIndex;
use lexer::{LexError, Lexer};
use std::fmt::{self, Display, Formatter};

pub type Spanned<'input> = Result<(ByteIndex, Token<'input>, ByteIndex), LexError>;

pub fn construct_lexer(src: &str) -> impl Iterator<Item = Spanned> {
    let mut lexer = Lexer::new(src).skipping(r"^[\t\v \r]+|(?m)^'.*$");

    // punctuation
    lexer.register_pattern(r"^=", |_| Token::Equals);
    lexer.register_pattern(r"^,", |_| Token::Comma);
    lexer.register_pattern(r"^:", |_| Token::Colon);
    lexer.register_pattern(r"^\(", |_| Token::OpenParen);
    lexer.register_pattern(r"^\)", |_| Token::CloseParen);
    lexer.register_pattern(r"^\n", |_| Token::EndOfLine);
    lexer.register_pattern(r"^\+", |_| Token::Plus);
    lexer.register_pattern(r"^-", |_| Token::Minus);
    lexer.register_pattern(r"^\*", |_| Token::Asterisk);

    // keywords
    lexer.register_pattern(r"^(?i)dim", |_| Token::Dim);
    lexer.register_pattern(r"^(?i)as", |_| Token::As);
    lexer.register_pattern(r"^(?i)print", |_| Token::Print);
    lexer.register_pattern(r"^(?i)goto", |_| Token::Goto);

    // literals
    lexer.register_pattern(r"^\d+\.\d+", |s| {
        Token::Float(s.parse().expect("parse never fails"))
    });
    lexer.register_pattern(r"^\d+", |s| {
        Token::Integer(s.parse().expect("parse never fails"))
    });
    lexer.register_pattern(r#"^"(?:[^"\\]|\\.)*""#, |s| {
        Token::String(&s[1..s.len() - 1])
    });

    // identifiers
    lexer.register_pattern(r"^[\w][\w\d_]*", |s| Token::Identifier(s));

    AppendEOL::new(lexer).fuse()
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token<'input> {
    Integer(i64),
    Float(f64),
    String(&'input str),
    Identifier(&'input str),

    // punctuation
    Colon,
    Comma,
    Equals,
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Asterisk,
    EndOfLine,

    // keywords
    Dim,
    As,
    Print,
    Goto,
}

impl<'input> Token<'input> {
    pub fn as_ident(&self) -> Option<&'input str> {
        match *self {
            Token::Identifier(id) => Some(id),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match *self {
            Token::Integer(i) => Some(i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match *self {
            Token::Float(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Token::String(ref s) => Some(s),
            Token::Print => Some("print"),
            Token::Goto => Some("goto"),
            _ => None,
        }
    }
}

impl<'input> From<&'input str> for Token<'input> {
    fn from(other: &'input str) -> Token<'input> {
        Token::Identifier(other)
    }
}

impl<'input> From<f64> for Token<'input> {
    fn from(other: f64) -> Token<'input> {
        Token::Float(other)
    }
}

impl<'input> From<i64> for Token<'input> {
    fn from(other: i64) -> Token<'input> {
        Token::Integer(other)
    }
}

impl<'input> Display for Token<'input> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Token::Integer(i) => i.fmt(f),
            Token::Float(fl) => fl.fmt(f),
            Token::String(s) => s.fmt(f),
            Token::Identifier(s) => s.fmt(f),
            Token::Colon => ":".fmt(f),
            Token::Comma => ",".fmt(f),
            Token::Equals => "=".fmt(f),
            Token::OpenParen => "(".fmt(f),
            Token::CloseParen => ")".fmt(f),
            Token::EndOfLine => "<end-of-line>".fmt(f),
            Token::Dim => "dim".fmt(f),
            Token::As => "as".fmt(f),
            Token::Print => "print".fmt(f),
            Token::Goto => "goto".fmt(f),
            Token::Plus => "+".fmt(f),
            Token::Minus => "-".fmt(f),
            Token::Asterisk => "*".fmt(f),
        }
    }
}

struct AppendEOL<I> {
    inner: I,
    last_loc: ByteIndex,
    fired: bool,
}

impl<I> AppendEOL<I> {
    fn new(stream: I) -> Self {
        AppendEOL {
            inner: stream,
            last_loc: ByteIndex(0),
            fired: false,
        }
    }
}

impl<'input, I> Iterator for AppendEOL<I>
where
    I: Iterator<Item = Spanned<'input>>,
{
    type Item = Spanned<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next();

        if let Some(Ok((_, _, loc))) = item {
            self.last_loc = loc;
        }

        if item.is_none() && !self.fired {
            self.fired = true;
            return Some(Ok((self.last_loc, Token::EndOfLine, self.last_loc)));
        }

        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codespan::Span;

    #[test]
    fn apostrophe_is_comment() {
        let got = construct_lexer("' this is a comment\n X")
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(got.len(), 3);
        let (_, x, _) = got[1];
        assert_eq!(x, Token::Identifier("X"));
    }

    #[test]
    fn lexer_ends_with_eol() {
        let got = construct_lexer("").collect::<Result<Vec<_>, _>>().unwrap();

        assert_eq!(got.len(), 1);

        let (_, tok, _) = got[0];

        assert_eq!(tok, Token::EndOfLine);
    }

    #[test]
    fn every_line_is_an_eol() {
        let src = "\n\n\n\n";

        let got = construct_lexer(src).collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(got.len(), 4 + 1);

        for (_, tok, _) in got {
            assert_eq!(tok, Token::EndOfLine);
        }
    }

    #[test]
    fn lexer_gives_correct_error_locations() {
        let src = "hello world!";
        let should_be = Span::new(ByteIndex(11), ByteIndex(12));

        let err = construct_lexer(src)
            .collect::<Result<Vec<_>, _>>()
            .unwrap_err();

        assert_eq!(err.span, should_be);
    }

    #[test]
    fn tokenize_a_string() {
        let src = "\"Hello World\"";
        let should_be = Token::String("Hello World");

        let (start, got, end) = construct_lexer(src).next().unwrap().unwrap();

        assert_eq!(got, should_be);
        assert_eq!(start, ByteIndex(0));
        assert_eq!(end, ByteIndex(src.len() as u32));
    }
}
