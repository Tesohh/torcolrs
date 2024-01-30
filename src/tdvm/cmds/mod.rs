use super::command::Command;

pub mod arithmetic;
pub mod lasa;
pub mod stampa;

pub fn commands() -> Vec<Command> {
    vec![
        stampa::stampa(),
        lasa::lasa(),
        arithmetic::jonta(),
        arithmetic::sotra(),
    ]
}
