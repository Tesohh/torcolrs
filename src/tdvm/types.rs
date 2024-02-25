use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum Type {
    Num,
    Str,
    Bool,
    Void,
    Block,
    Type,
    Ident,
    Any,
}

impl Into<Type> for String {
    fn into(self) -> Type {
        match self.as_str() {
            "Num" => Type::Num,
            "Str" => Type::Str,
            "Bool" => Type::Bool,
            "Void" => Type::Void,
            "Block" => Type::Block,
            "Type" => Type::Type,
            "Ident" => Type::Ident,
            "Any" => Type::Any,
            &_ => unreachable!(),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Num => write!(f, "Num"),
            Type::Str => write!(f, "Str"),
            Type::Bool => write!(f, "Bool"),
            Type::Void => write!(f, "Void"),
            Type::Block => write!(f, "Block"),
            Type::Type => write!(f, "Type"),
            Type::Ident => write!(f, "Ident"),
            Type::Any => write!(f, "Any"),
        }
    }
}
