use anyhow::Context;

use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest, Command},
        types::Type,
        value::{Extract, Value},
    },
};

pub fn greater() -> Command {
    Command {
        name: "greater".into(),
        requested_args: args!(lhs: Num, rhs: Num),
        inner: |args, _| {
            let lhs = args.get(0).context("lhs")?.extract_num()?;
            let rhs = args.get(1).context("lhs")?.extract_num()?;

            Ok(Value::Bool(lhs > rhs))
        },
    }
}
pub fn lesser() -> Command {
    Command {
        name: "lesser".into(),
        requested_args: args!(lhs: Num, rhs: Num),
        inner: |args, _| {
            let lhs = args.get(0).context("lhs")?.extract_num()?;
            let rhs = args.get(1).context("lhs")?.extract_num()?;

            Ok(Value::Bool(lhs < rhs))
        },
    }
}

// TODO: ADD TESTS
