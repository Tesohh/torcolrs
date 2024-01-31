use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Num,
    Str,
    Bool,
    Void,
    Block,
    Any,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Num => write!(f, "Num"),
            Type::Str => write!(f, "Str"),
            Type::Bool => write!(f, "Bool"),
            Type::Void => write!(f, "Void"),
            Type::Block => write!(f, "Block"),
            Type::Any => write!(f, "Any"),
        }
    }
}
