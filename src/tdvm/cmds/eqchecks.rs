use anyhow::Context;

use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest, Command, Inner},
        types::Type,
        value::Value,
    },
};

pub fn eq() -> Command {
    Command {
        // valif
        name: "eq".into(),
        requested_args: args!(lhs: Any, rhs: Any),
        inner: Inner::Rusty(|args, _| {
            let lhs = args.get(0).context("lhs")?;
            let rhs = args.get(1).context("lhs")?;

            Ok(Value::Bool(*lhs == *rhs))
        }),
    }
}
pub fn neq() -> Command {
    Command {
        // desvalif
        name: "neq".into(),
        requested_args: args!(lhs: Any, rhs: Any),
        inner: Inner::Rusty(|args, _| {
            let lhs = args.get(0).context("lhs")?;
            let rhs = args.get(1).context("lhs")?;

            Ok(Value::Bool(*lhs != *rhs))
        }),
    }
}

// TODO: ADD TESTS
