use super::command::Command;

pub mod arithmetic;
pub mod cmd;
pub mod eqchecks;
pub mod greaterlesser;
pub mod lasa;
pub mod no;
pub mod se;
pub mod stampa;
pub mod testlasa;
pub mod treturn;

pub fn commands() -> Vec<Command> {
    vec![
        stampa::stampa(),
        lasa::lasa(),
        testlasa::testlasa(),
        se::se(),
        arithmetic::jonta(),
        arithmetic::sotra(),
        eqchecks::eq(),
        eqchecks::neq(),
        greaterlesser::greater(),
        greaterlesser::lesser(),
        cmd::cmd(),
        treturn::treturn(),
        no::no(),
    ]
}
