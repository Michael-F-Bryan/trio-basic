//! Internal bytecode representation.

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    rust_2018_idioms
)]

use slog::{self, Serializer, Value};
use std::fmt::{self, Debug, Display, Formatter};

/// A fully compiled bytecode program.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Program {
    /// Statically-allocated strings.
    pub string_table: Vec<String>,
    /// The program's entrypoint (i.e. `main()`).
    pub entrypoint: FunctionIndex,
    /// All user-defined functions "linked" into this [`Program`].
    pub functions: Vec<Function>,
}

/// A single stack-machine instruction.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    /// Call a user-defined function.
    CallUserFunction {
        /// Which function to call.
        function: FunctionIndex,
    },
    /// Call a builtin function.
    CallBuiltinFunction {
        /// The name of the function to call.
        function: StringIndex,
        /// The number of arguments to pass to the function.
        args: usize,
    },
    /// Jump to a labeled instruction within the current function.
    Goto {
        /// The label to jump to.
        label: LabelIndex,
    },
    /// Push an `[i32`] onto the top of the stack.
    PushInteger(i32),
    /// Push a [`f64`] onto the top of the stack.
    PushDouble(f64),
    /// Push a [`bool`] onto the top of the stack.
    PushBoolean(bool),
    /// Push a string from the string table to the top of the stack.
    PushString {
        /// The string to push onto the stack.
        string: StringIndex,
    },
    /// Copy the value from a global variable onto the stack.
    LoadGlobal {
        /// An index into the string table with the variable's name.
        variable: StringIndex,
    },
    /// Pop a value from the top of the stack and save it in a global variable.
    StoreGlobal {
        /// The variable's name from the string table.
        variable: StringIndex,
    },
    /// Pop an item off the top of the stack.
    Pop,
    /// Pop an item from the stack, goto to the `true_label` if it is `true`,
    /// otherwise goto to the `false_label`.
    Branch {
        /// The label to jump to if the condition evaluates to `true`.
        true_label: LabelIndex,
        /// The label to jump to if the condition evaluates to `false`.
        false_label: LabelIndex,
    },
    /// Return control to the calling function.
    Return,
    /// Add two values.
    ///
    /// The operands must be the same type, and either [`Type::Integer`] or
    /// [`Type::Double`]
    Add,
}

/// A function definition.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Function {
    /// The function's name.
    pub name: String,
    /// The type
    pub return_ty: Option<Type>,
    /// The function's body.
    pub body: Vec<Instruction>,
    /// Labels which have been defined within the current function.
    pub labels: Vec<usize>,
}

/// The builtin types.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    /// The [`bool`] primitive.
    Boolean,
    /// The [`i32`] primitive.
    Integer,
    /// The [`f64`] primitive.
    Double,
    /// The [`String`] primitive.
    String,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result { Debug::fmt(self, f) }
}

/// A poor man's `std::slice::SliceExt` because the original trait is sealed.
pub trait SliceExt<Index> {
    /// The retrieved value.
    type Output;

    /// Get the value.
    fn get_(&self, index: Index) -> Option<&Self::Output>;
}

macro_rules! newtype_index {
    ($( #[$attr:meta] )* $name:ident -> $slice:ty) => {
        $(#[$attr])*
        #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name(pub usize);

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                Debug::fmt(self, f)
            }
        }

        impl Value for $name {
            fn serialize(
                &self,
                _record: &slog::Record<'_>,
                key: &'static str,
                ser: &mut dyn Serializer,
            ) -> slog::Result {
                ser.emit_usize(key, self.0)
            }
        }

        impl From<usize> for $name {
            fn from(other: usize) -> $name { $name(other) }
        }

        impl SliceExt<$name> for [$slice] {
            type Output = $slice;

            fn get_(&self, index: $name) -> Option<&Self::Output> {
                self.get(index.0)
            }
        }

        impl std::ops::Index<$name> for Vec<$slice> {
            type Output = $slice;

            fn index(&self, ix: $name) -> &Self::Output { self.index(ix.0) }
        }
    };
}

newtype_index! {
    /// A strongly-typed index into the [`Program::string_table`].
    StringIndex -> String
}
newtype_index! {
    /// A strongly-typed index into [`Function::labels`].
    LabelIndex -> usize
}
newtype_index! {
    /// A strongly-typed index into [`Program::functions`].
    FunctionIndex -> Function
}
