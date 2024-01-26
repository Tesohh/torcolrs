use super::types::Type;

#[derive(Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<Value>),
    Void,
    // Arg((String, Type))
}

impl Into<Type> for Value {
    fn into(self) -> Type {
        match self {
            Value::Bool(_) => Type::Bool,
            Value::Num(_) => Type::Num,
            Value::Str(_) => Type::Str,
            Value::Array(_) => todo!(),
            Value::Void => Type::Void,
        }
    }
}

impl Into<Type> for &Value {
    fn into(self) -> Type {
        match self {
            Value::Bool(_) => Type::Bool,
            Value::Num(_) => Type::Num,
            Value::Str(_) => Type::Str,
            Value::Array(_) => todo!(),
            Value::Void => Type::Void,
        }
    }
}
