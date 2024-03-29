use std::fmt::Display;

use anyhow::bail;

use super::types::Type;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<Value>),
    Block(String),
    Type(Type),
    Ident(String),
    Void,
    // Arg((String, Type))
}

trait Name {
    fn name(&self) -> &str;
}

impl Name for Value {
    fn name(&self) -> &str {
        match self {
            Value::Bool(_) => "Bool",
            Value::Num(_) => "Num",
            Value::Str(_) => "Str",
            Value::Array(_) => "Array",
            Value::Block(_) => "Block",
            Value::Type(_) => "Type",
            Value::Ident(_) => "Ident",
            Value::Void => "Void",
        }
    }
}

pub trait Extract {
    fn extract_bool(&self) -> anyhow::Result<bool>;
    fn extract_num(&self) -> anyhow::Result<f64>;
    fn extract_str(&self) -> anyhow::Result<String>;
    fn extract_array(&self) -> anyhow::Result<Vec<Value>>;
    fn extract_type(&self) -> anyhow::Result<Type>;
    fn extract_block(&self) -> anyhow::Result<String>;
}

impl Extract for Value {
    fn extract_bool(&self) -> anyhow::Result<bool> {
        match self {
            Value::Bool(v) => Ok(*v),
            _ => bail!("cannot extract Bool from {}", self.name()),
        }
    }

    fn extract_num(&self) -> anyhow::Result<f64> {
        match self {
            Value::Num(v) => Ok(*v),
            _ => bail!("cannot extract Num from {}", self.name()),
        }
    }

    fn extract_str(&self) -> anyhow::Result<String> {
        match self {
            Value::Str(v) => Ok(v.into()),
            Value::Ident(v) => Ok(v.into()),
            _ => bail!("cannot extract Str from {}", self.name()),
        }
    }

    fn extract_array(&self) -> anyhow::Result<Vec<Value>> {
        match self {
            Value::Array(v) => Ok(v.to_vec()),
            _ => bail!("cannot extract Array from {}", self.name()),
        }
    }

    fn extract_type(&self) -> anyhow::Result<Type> {
        match self {
            Value::Type(v) => Ok(v.clone()),
            _ => bail!("cannot extract Type from {}", self.name()),
        }
    }

    fn extract_block(&self) -> anyhow::Result<String> {
        match self {
            Value::Block(v) => Ok(v.into()),
            _ => bail!("cannot extract Block from {}", self.name()),
        }
    }
}

impl Extract for &Value {
    fn extract_bool(&self) -> anyhow::Result<bool> {
        (*self).extract_bool()
    }
    fn extract_num(&self) -> anyhow::Result<f64> {
        (*self).extract_num()
    }
    fn extract_str(&self) -> anyhow::Result<String> {
        (*self).extract_str()
    }
    fn extract_array(&self) -> anyhow::Result<Vec<Value>> {
        (*self).extract_array()
    }
    fn extract_type(&self) -> anyhow::Result<Type> {
        (*self).extract_type()
    }
    fn extract_block(&self) -> anyhow::Result<String> {
        (*self).extract_block()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(v) => {
                if *v {
                    write!(f, "vera")
                } else {
                    write!(f, "faus")
                }
            }
            Value::Num(v) => write!(f, "{}", v),
            Value::Str(v) => write!(f, "{}", v),
            Value::Block(_) => write!(f, "[bloch]"),
            Value::Type(v) => write!(f, "{}", v),
            Value::Ident(v) => write!(f, "Identifier {}", v),
            Value::Array(_) => todo!(),
            Value::Void => write!(f, "vet"),
        }
    }
}

impl Into<Type> for Value {
    fn into(self) -> Type {
        match self {
            Value::Bool(_) => Type::Bool,
            Value::Num(_) => Type::Num,
            Value::Str(_) => Type::Str,
            Value::Array(_) => todo!(),
            Value::Block(_) => Type::Block,
            Value::Ident(_) => Type::Ident,
            Value::Type(_) => Type::Type,
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
            Value::Block(_) => Type::Block,
            Value::Ident(_) => Type::Ident,
            Value::Type(_) => Type::Type,
            Value::Void => Type::Void,
        }
    }
}
