use anyhow::Context;

use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest, Command, Inner},
        types::Type,
        value::Value,
    },
};

pub fn stampa() -> Command {
    Command {
        name: "stampa".into(),
        requested_args: args!(val: Any),
        inner: Inner::Rusty(|args, _tdvm| {
            println!("{}", args.get(0).context("first arg is empty")?);
            Ok(Value::Void)
        }),
    }
}
