#![allow(unused)]

use crate::tdvm::{types::Type, value::Value}; // TODO: remove this
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Cmd(String),
    Var(String),
    Comment(String),
    Value(Value),
    Sub(Tokens),
    // Num(f64),
    // Str(String),
    // Bool(bool),
    Ident(String),
    Type(Type),
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
