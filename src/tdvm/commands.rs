use anyhow::{bail, Context, Ok};

use crate::tdvm::{
    command::{Arg, ArgsRequest},
    types::Type,
    value::{Extract, Value},
};

use super::command::Command;

pub fn commands() -> Vec<Command> {
    vec![
        Command {
            name: "stampa".into(),
            requested_args: ArgsRequest::Limited(vec![Arg {
                name: "s".into(),
                expected: Type::Any,
            }]),
            inner: |args, _tdvm| {
                println!("{}", args.get(0).context("not found")?);
                Ok(Value::Void)
            },
        },
        Command {
            name: "jonta".into(),
            requested_args: ArgsRequest::Limited(vec![
                Arg {
                    name: "num1".into(),
                    expected: Type::Num,
                },
                Arg {
                    name: "num2".into(),
                    expected: Type::Num,
                },
            ]),
            inner: |args, _tdvm| {
                let args = (args.get(0).context("num1")?, args.get(1).context("num1")?);
                let n1 = args.0.extract_num()?;
                let n2 = args.1.extract_num()?;
                Ok(Value::Num(n1 + n2))
            },
        },
        Command {
            name: "sotra".into(),
            requested_args: ArgsRequest::Limited(vec![
                Arg {
                    name: "num1".into(),
                    expected: Type::Num,
                },
                Arg {
                    name: "num2".into(),
                    expected: Type::Num,
                },
            ]),
            inner: |args, _tdvm| {
                let args = (args.get(0).context("num1")?, args.get(1).context("num1")?);
                let mut sum = 0.0;
                if let (Value::Num(n1), Value::Num(n2)) = args {
                    sum = n1 - n2;
                }
                Ok(Value::Num(sum))
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
