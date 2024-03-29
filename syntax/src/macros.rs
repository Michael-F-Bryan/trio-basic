/// A shortcut for creating an enum such as
///
/// ```
/// enum Foo {
///   Bar(Bar),
///   Baz(Baz),
///   ...
/// }
/// ```
macro_rules! enum_decl {
    // shortcut for when the variant and its type are the same
    ($( #[$attr:meta]  )* $name:ident => $( $variant:tt, )*) => {
        enum_decl!{ $( #[$attr] )* $name => $( $variant($variant),)* }
    };
    // defer to the main implementation, but also implement AstNode
    (AstNode $( #[$attr:meta]  )* $name:ident => $( $variant:ident, )*) => {
        enum_decl!{ $( #[$attr] )* $name => $( $variant($variant),)* }

        impl $crate::ast::AstNode for $name {
            fn span(&self) -> codespan::Span {
                match *self {
                    $(
                        $name::$variant(ref item) => item.span()
                    ),*
                }
            }

            fn span_mut(&mut self) -> &mut codespan::Span {
                match *self {
                    $(
                        $name::$variant(ref mut item) => item.span_mut()
                    ),*
                }
            }
        }
    };
    ($( #[$attr:meta]  )* $name:ident => $( $variant:ident ($type:ty), )*) => {
        #[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
        #[serde(rename_all = "kebab-case")]
        $(
            #[$attr]
        )*
        pub enum $name {
            $(
                $variant($type)
            ),*
        }

        $(
            impl From<$type> for $name {
                fn from(other: $type) -> $name {
                    $name::$variant(other)
                }
            }
        )*
    };
}

macro_rules! impl_from_str {
    ($type:ty, $parser:ident) => {
        impl $type {
            /// Try to parse the provided string as this type of AST node.
            pub fn parse(
                src: &str,
            ) -> ::std::result::Result<Self, $crate::ParseError> {
                let src = $crate::tokens::construct_lexer(src);
                $crate::grammar::$parser::new()
                    .parse(src)
                    .map_err($crate::ParseError::from)
            }
        }
    };
}

/// Implement the `AstNode` trait automatically.
macro_rules! impl_ast_node {
    ($name:ident) => {
        impl_ast_node!($name;);
    };
    ($name:ident; $( $defer:ident ),*) => {
        impl_ast_node!($name; $( $defer ),*;);
    };
    ($name:ident;
    $( $defer:ident ),*;
    $( $loop_defer:ident ),*) => {
        impl $crate::ast::AstNode for $name {
            fn span(&self) -> codespan::Span {
                self.span
            }

            fn span_mut(&mut self) -> &mut codespan::Span {
                &mut self.span
            }
        }
    };
}
