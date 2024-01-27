use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum Type {
    Num,
    Str,
    Bool,
    Void,
    Any,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Num => write!(f, "Num"),
            Type::Str => write!(f, "Str"),
            Type::Bool => write!(f, "Bool"),
            Type::Void => write!(f, "Void"),
            Type::Any => write!(f, "Any"),
        }
    }
}
