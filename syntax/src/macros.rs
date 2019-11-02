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
            fn span(&self) -> ::codespan::ByteSpan {
                match *self {
                    $(
                        $name::$variant(ref item) => item.span()
                    ),*
                }
            }

            fn span_mut(&mut self) -> &mut ::codespan::ByteSpan {
                match *self {
                    $(
                        $name::$variant(ref mut item) => item.span_mut()
                    ),*
                }
            }

            fn offset_inplace(&mut self, offset: ByteOffset) {
                match *self {
                    $(
                        $name::$variant(ref mut item) => item.offset_inplace(offset)
                    ),*
                }
            }
        }
    };
    ($( #[$attr:meta]  )* $name:ident => $( $variant:ident ($type:ty), )*) => {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, HeapSizeOf)]
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

/// Implement the `HeapSizeOf` trait for a struct by destructuring it and
/// adding up the heap size of its elements.
#[macro_export]
macro_rules! impl_heapsize {
    ($type:ident : $( $field:ident ),* ) => {
        #[allow(unused_mut)]
        impl HeapSizeOf for $type {
            fn heap_size_of_children(&self) -> usize {
                let $type {
                    $( ref $field, )*
                    span: _span,
                } = *self;

                let mut size = 0;

                $(
                    size += $field.heap_size_of_children();
                )*

                size
            }
        }
    };
    ($type:ident) => {
        impl_heapsize!($type :);
    };
}

macro_rules! impl_from_str {
    ($type:ty, $parser:ident) => {
        impl $type {
            /// Try to parse the provided string as this type of AST node.
            pub fn from_str(src: &str) -> ::std::result::Result<Self, $crate::ParseError> {
                let src = $crate::tokens::construct_lexer(src);
                $crate::grammar::$parser::new()
                    .parse(src)
                    .map_err(|e| $crate::ParseError::from(e))
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
            fn span(&self) -> ::codespan::ByteSpan {
                self.span
            }

            fn span_mut(&mut self) -> &mut ::codespan::ByteSpan {
                &mut self.span
            }

            fn offset_inplace(&mut self, offset: ::codespan::ByteOffset) {
                self.span = self.span.map(|ix| ix + offset);

                $(
                    self.$defer.offset_inplace(offset);
                )*

                $(
                    for item in &mut self.$loop_defer {
                        item.offset_inplace(offset);
                    }
                )*
            }
        }
    };
}
