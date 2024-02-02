use super::command::Command;

pub mod arithmetic;
pub mod lasa;
pub mod se;
pub mod stampa;
pub mod testlasa;

pub fn commands() -> Vec<Command> {
    vec![
        stampa::stampa(),
        lasa::lasa(),
        testlasa::testlasa(),
        se::se(),
        arithmetic::jonta(),
        arithmetic::sotra(),
    ]
}
