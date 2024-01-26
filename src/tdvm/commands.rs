use anyhow::{bail, Context};

use crate::tdvm::{
    command::{Arg, ArgsRequest},
    tdvm::Tdvm,
    types::Type,
    value::Value,
};

use super::command::Command;

pub fn commands() -> Vec<Command> {
    vec![Command {
        name: "stampa".into(),
        requested_args: ArgsRequest::Limited(vec![Arg {
            name: "s".into(),
            expected: Type::Str,
        }]),
        inner: |args, _tdvm| {
            if let Value::Str(s) = args.get(0).context("not found")? {
                println!("{}", s);
                Ok(Value::Void)
            } else {
                bail!("wrong arg")
            }
        },
    }]
}
