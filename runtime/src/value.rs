use bytecode::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Integer(i32),
    Double(f64),
    String(String),
}

impl Value {
    pub fn ty(&self) -> Type {
        match self {
            Value::Boolean(_) => Type::Boolean,
            Value::Integer(_) => Type::Integer,
            Value::Double(_) => Type::Double,
            Value::String(_) => Type::String,
        }
    }

    pub fn default_value(ty: Type) -> Value {
        match ty {
            Type::Boolean => Value::Boolean(false),
            Type::Integer => Value::Integer(0),
            Type::Double => Value::Double(0.0),
            Type::String => Value::String(String::new()),
            Type::Void => unreachable!(
                "It's not possible to create an instance of `void` at runtime"
            ),
        }
    }
}
