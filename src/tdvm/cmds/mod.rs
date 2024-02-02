use super::command::Command;

pub mod arithmetic;
pub mod lasa;
pub mod se;
pub mod stampa;

pub fn commands() -> Vec<Command> {
    vec![
        stampa::stampa(),
        lasa::lasa(),
        se::se(),
        arithmetic::jonta(),
        arithmetic::sotra(),
    ]
}
