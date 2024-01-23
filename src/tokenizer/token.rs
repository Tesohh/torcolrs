#[derive(Debug, PartialEq)]
pub enum Token {
    Cmd(String),
    Var(String),
    Num(f64),
    Str(String),
    ParOpen,
    ParClose,
    SquirlyOpen,
    SquirlyClose,
}

pub type Tokens = Vec<Token>;
