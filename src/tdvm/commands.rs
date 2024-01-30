use anyhow::{bail, Context, Ok};

use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest},
        types::Type,
        value::{Extract, Value},
        var::Var,
    },
};

use super::command::Command;

pub fn commands() -> Vec<Command> {
    vec![
        Command {
            name: "stampa".into(),
            requested_args: args!(val: Any),
            inner: |args, _tdvm| {
                println!("{}", args.get(0).context("not found")?);
                Ok(Value::Void)
            },
        },
        Command {
            name: "jonta".into(),
            requested_args: args!(num1: Num, num2: Num),
            inner: |args, _tdvm| {
                let args = (args.get(0).context("num1")?, args.get(1).context("num2")?);
                let n1 = args.0.extract_num()?;
                let n2 = args.1.extract_num()?;
                Ok(Value::Num(n1 + n2))
            },
        },
        Command {
            name: "sotra".into(),
            requested_args: args!(num1: Num, num2: Num),
            inner: |args, _tdvm| {
                let args = (args.get(0).context("num1")?, args.get(1).context("num2")?);
                let n1 = args.0.extract_num()?;
                let n2 = args.1.extract_num()?;
                Ok(Value::Num(n1 - n2))
            },
        },
        Command {
            name: "lasa".into(),
            requested_args: args!(name: Str, value: Any),
            inner: |args, tdvm| {
                let name = args.get(0).context("name")?.extract_str()?;
                let value = args.get(1).context("value")?;
                tdvm.memory.insert(
                    name,
                    Var {
                        value: value.clone(),
                    },
                );
                dbg!(&tdvm.memory);

                Ok(value.clone())
            },
        },
    ]
}
