#![allow(unused)] // TODO: remove this
#[derive(Debug, PartialEq)]
pub enum Token {
    Cmd(String),
    Var(String),
    Type(String),
    Comment(String),
    Num(f64),
    Str(String),
    Bool(bool),
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
