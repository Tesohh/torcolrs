use anyhow::{bail, Context, Ok};

use crate::tdvm::{
    command::{Arg, ArgsRequest},
    types::Type,
    value::Value,
};

use super::command::Command;

pub fn commands() -> Vec<Command> {
    vec![
        Command {
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
        },
        // TODO:
        Command {
            name: "lasa".into(),
            requested_args: ArgsRequest::Void,
            inner: |_args, _tdvm| Ok(Value::Void),
        },
    ]
}
