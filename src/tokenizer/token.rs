#![allow(unused)]

use crate::tdvm::tdvm::Value; // TODO: remove this
#[derive(Debug, PartialEq)]
pub enum Token {
    Cmd(String),
    Var(String),
    Type(String),
    Comment(String),
    Value(Value),
    // Num(f64),
    // Str(String),
    // Bool(bool),
    Unknown(String),
    ParOpen,
    ParClose,
    SquirlyOpen,
    SquirlyClose,
    GreaterThan,
    LessThan,
    Equals,
}

pub type Tokens = Vec<Token>;
