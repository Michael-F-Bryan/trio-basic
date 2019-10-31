//! Internal bytecode representation.

/// A fully compiled bytecode program.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    /// Statically-allocated strings.
    pub string_table: Vec<String>,
    pub entrypoint: FunctionID,
    /// All user-defined functions "linked" into this [`Program`].
    pub functions: Vec<Function>,
}

pub type StringIndex = usize;
pub type LabelIndex = usize;
pub type FunctionID = usize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    /// Call a user-defined function.
    CallUserFunction {
        /// Which function to call.
        function_id: FunctionID,
    },
    /// Call a builtin function.
    CallBuiltinFunction {
        /// Which function to call.
        function_id: FunctionID,
        /// The number of arguments to pass to the function.
        args: usize,
    },
    /// Jump to a labeled instruction within the current function.
    Goto { label: LabelIndex },
    /// Push an `[i32`] onto the top of the stack.
    PushInteger(i32),
    /// Push a [`f64`] onto the top of the stack.
    PushDouble(f64),
    /// Push a [`bool`] onto the top of the stack.
    PushBoolean(bool),
    /// Push a string from the string table to the top of the stack.
    PushString { string_id: StringIndex },
    /// Copy the value from a global variable onto the stack.
    PushGlobal {
        /// An index into the string table with the variable's name.
        variable_id: StringIndex,
    },
    /// Pop a value from the top of the stack and save it in a global variable.
    PopGlobal { variable_id: StringIndex },
    /// Pop an item off the top of the stack.
    Pop,
    /// Pop an item from the stack, goto to the `true_label` if it is `true`,
    /// otherwise goto to the `false_label`.
    Branch {
        true_label: LabelIndex,
        false_label: LabelIndex,
    },
    /// Return control to the calling function.
    Return,
}

/// A function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    /// The function's name.
    pub name: String,
    pub return_ty: Option<Type>,
    /// The function's body.
    pub body: Vec<Instruction>,
    pub labels: Vec<usize>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Boolean,
    Integer,
    Double,
    String,
}
