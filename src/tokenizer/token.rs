#[derive(Debug, PartialEq)]
pub enum Token {
    Cmd(String),
    Var(String),
    Comment(String),
    Num(f64),
    Str(String),
    ParOpen,
    ParClose,
    SquirlyOpen,
    SquirlyClose,
    GreaterThan,
    LessThan,
    Equals,
}

pub type Tokens = Vec<Token>;
