use anyhow::Context;

use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest, Command},
        types::Type,
        value::Value,
    },
};

pub fn stampa() -> Command {
    Command {
        name: "stampa".into(),
        requested_args: args!(val: Any),
        inner: |args, _tdvm| {
            println!("{}", args.get(0).context("not found")?);
            Ok(Value::Void)
        },
    }
}
