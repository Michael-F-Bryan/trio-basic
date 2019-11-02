//! The various types which make up a TRIO Basic program's AST.

use codespan::Span;
use heapsize::HeapSizeOf;
use heapsize_derive::HeapSizeOf;
use serde_derive::{Deserialize, Serialize};

pub trait AstNode {
    fn span(&self) -> Span;
    fn span_mut(&mut self) -> &mut Span;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub lines: Vec<Line>,
    pub span: Span,
}

impl File {
    pub fn new(lines: Vec<Line>, span: Span) -> File { File { lines, span } }
}

/// A variable, type, or function name.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Ident {
    pub fn new<S: Into<String>>(name: S, span: Span) -> Ident {
        Ident {
            name: name.into(),
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Literal {
    pub kind: LiteralKind,
    pub span: Span,
}

impl Literal {
    pub fn new<L: Into<LiteralKind>>(kind: L, span: Span) -> Literal {
        Literal {
            kind: kind.into(),
            span,
        }
    }
}

/// `foo(a, b, 42)`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: Ident,
    pub args: Vec<Expr>,
    pub span: Span,
}

impl FunctionCall {
    pub fn new(name: Ident, args: Vec<Expr>, span: Span) -> FunctionCall {
        FunctionCall { name, args, span }
    }
}

/// `start:`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    pub span: Span,
}

impl Label {
    pub fn new<S: Into<String>>(name: S, span: Span) -> Label {
        let name = name.into();
        Label { name, span }
    }
}

// `x = 5`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    pub ident: Ident,
    pub value: Expr,
    pub span: Span,
}

impl Assignment {
    pub fn new(ident: Ident, value: Expr, span: Span) -> Assignment {
        Assignment { ident, value, span }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dim {
    pub name: Ident,
    pub ty: Ident,
    pub span: Span,
}

impl Dim {
    pub fn new(name: Ident, ty: Ident, span: Span) -> Dim {
        Dim { name, ty, span }
    }
}

enum_decl! {
    AstNode
    /// A single line.
    Line => Label, Assignment, Dim, FunctionCall, Statement,
}
enum_decl! {
    AstNode
    /// An expression (something that has a type and value).
    Expr => Ident, Literal, FunctionCall, BinaryOp,
}
enum_decl! {
    LiteralKind => Integer(i64), Float(f64), String(String),
}

impl<'a> From<&'a str> for LiteralKind {
    fn from(other: &'a str) -> LiteralKind {
        LiteralKind::String(other.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryOp {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: Operator,
    pub span: Span,
}

impl BinaryOp {
    pub fn new(left: Expr, right: Expr, op: Operator, span: Span) -> BinaryOp {
        BinaryOp {
            left: Box::new(left),
            right: Box::new(right),
            op,
            span,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, HeapSizeOf)]
pub enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statement {
    pub name: String,
    pub args: Vec<Expr>,
    pub span: Span,
}

impl Statement {
    pub fn new<S: Into<String>>(
        name: S,
        args: Vec<Expr>,
        span: Span,
    ) -> Statement {
        Statement {
            name: name.into(),
            args,
            span,
        }
    }
}

// codespan doesn't implement the HeapSizeOf trait, so we have to implement it
// manually instead of using the derive.
impl_heapsize!(Dim: name, ty);
impl_heapsize!(Assignment: ident, value);
impl_heapsize!(Label: name);
impl_heapsize!(FunctionCall: name, args);
impl_heapsize!(Literal: kind);
impl_heapsize!(Ident: name);
impl_heapsize!(File: lines);
impl_heapsize!(BinaryOp: left, right, op);
impl_heapsize!(Statement: name, args);

impl_from_str!(Assignment, AssignmentParser);
impl_from_str!(Dim, DimParser);
impl_from_str!(Expr, ExprParser);
impl_from_str!(File, FileParser);
impl_from_str!(FunctionCall, FunctionCallParser);
impl_from_str!(Ident, IdentParser);
impl_from_str!(Label, LabelParser);
impl_from_str!(Line, LineParser);
impl_from_str!(Literal, LiteralParser);

impl_ast_node!(Literal);
impl_ast_node!(Ident);
impl_ast_node!(File; ; lines);
impl_ast_node!(FunctionCall; name; args);
impl_ast_node!(Dim; name, ty);
impl_ast_node!(Label);
impl_ast_node!(Assignment; ident, value);
impl_ast_node!(BinaryOp; left, right);
impl_ast_node!(Statement; ;args);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        grammar::{ExprParser, FileParser, LineParser},
        tokens::{self, Spanned, Token},
    };
    use codespan::ByteIndex;

    macro_rules! tokens {
        ($( $token:expr ),*) => {{
            let items: Vec<Token> = vec![
                $(
                    $token.into()
                ),*
            ];

            tokenize(items)
        }};
    }

    #[test]
    fn parse_a_literal() {
        let src = tokens!(123.4);

        let should_be =
            Literal::new(123.4, Span::new(ByteIndex(0), ByteIndex(0)));
        let should_be = Expr::Literal(should_be);

        let got = ExprParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    fn tokenize<'input, I>(tokens: I) -> impl Iterator<Item = Spanned<'input>>
    where
        I: IntoIterator<Item = Token<'input>>,
    {
        tokens
            .into_iter()
            .map(|t| Ok((ByteIndex(0), t, ByteIndex(0))))
    }

    #[test]
    fn parse_an_ident() {
        let src = tokens!("foo");

        let should_be =
            Ident::new("foo", Span::new(ByteIndex(0), ByteIndex(0)));
        let should_be = Expr::Ident(should_be);

        let got = ExprParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_a_label() {
        let src = tokens::construct_lexer("foo:");

        let should_be =
            Label::new("foo", Span::new(ByteIndex(0), ByteIndex(4)));

        let got = LineParser::new().parse(src).unwrap();
        assert_eq!(got, Line::from(should_be));
    }

    #[test]
    fn parse_an_assignment() {
        let src = tokens::construct_lexer("foo = 42");

        let name = Ident::new("foo", Span::new(ByteIndex(0), ByteIndex(3)));
        let value = Literal::new(
            LiteralKind::Integer(42),
            Span::new(ByteIndex(6), ByteIndex(8)),
        );
        let should_be = Assignment::new(
            name,
            value.into(),
            Span::new(ByteIndex(0), ByteIndex(8)),
        );

        let got = LineParser::new().parse(src).unwrap();
        assert_eq!(got, Line::from(should_be));
    }

    #[test]
    fn parse_a_variable_declaration() {
        let src = tokens::construct_lexer("dim x as integer");

        let name = Ident::new("x", Span::new(ByteIndex(4), ByteIndex(5)));
        let ty = Ident::new("integer", Span::new(ByteIndex(9), ByteIndex(16)));
        let should_be =
            Dim::new(name, ty, Span::new(ByteIndex(0), ByteIndex(16)));

        let got = LineParser::new().parse(src).unwrap();
        assert_eq!(got, Line::from(should_be));
    }

    #[test]
    fn parse_multiple_lines() {
        let src = "start:\nx = 5\n";
        let src = tokens::construct_lexer(src);

        let got = FileParser::new().parse(src).unwrap();
        assert_eq!(got.lines.len(), 2);
    }

    #[test]
    fn parse_a_print_statement() {
        let src = "PRINT x, y";
        let should_be = Statement::new(
            "print",
            vec![
                Expr::Ident(Ident::new(
                    "x",
                    Span::new(ByteIndex(6), ByteIndex(7)),
                )),
                Expr::Ident(Ident::new(
                    "y",
                    Span::new(ByteIndex(9), ByteIndex(10)),
                )),
            ],
            Span::new(ByteIndex(0), ByteIndex(10)),
        );

        let got = Line::from_str(src).unwrap();

        assert_eq!(got, Line::Statement(should_be));
    }

    #[test]
    fn add_two_numbers() {
        let src = tokens!("x", Token::Plus, "y");
        let should_be = BinaryOp::new(
            Expr::Ident(Ident::new("x", Span::default())),
            Expr::Ident(Ident::new("y", Span::default())),
            Operator::Add,
            Span::default(),
        );

        let got = ExprParser::new().parse(src).unwrap();

        assert_eq!(got, Expr::BinaryOp(should_be));
    }
}
