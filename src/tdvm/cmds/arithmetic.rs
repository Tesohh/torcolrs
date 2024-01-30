use anyhow::Context;

use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest, Command},
        types::Type,
        value::{Extract, Value},
    },
};

pub fn jonta() -> Command {
    Command {
        name: "jonta".into(),
        requested_args: args!(num1: Num, num2: Num),
        inner: |args, _tdvm| {
            let args = (args.get(0).context("num1")?, args.get(1).context("num2")?);
            let n1 = args.0.extract_num()?;
            let n2 = args.1.extract_num()?;
            Ok(Value::Num(n1 + n2))
        },
    }
}

pub fn sotra() -> Command {
    Command {
        name: "sotra".into(),
        requested_args: args!(num1: Num, num2: Num),
        inner: |args, _tdvm| {
            let args = (args.get(0).context("num1")?, args.get(1).context("num2")?);
            let n1 = args.0.extract_num()?;
            let n2 = args.1.extract_num()?;
            Ok(Value::Num(n1 - n2))
        },
    }
}
