#[derive(Debug)]
pub enum Token {
    Cmd(String),
    Var(String),
    Num(isize),
    Str(String),
    ParOpen,
    ParClose,
    SquirlyOpen,
    SquirlyClose,
}

pub type Tokens = Vec<Token>;
