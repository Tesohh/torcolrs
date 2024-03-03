use anyhow::Context;

use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest, Command, Inner},
        types::Type,
        value::Value,
        var::Var,
    },
};

pub fn treturn() -> Command {
    Command {
        name: "return".into(),
        requested_args: args!(value: Any),
        inner: Inner::Rusty(|args, tdvm| {
            let value = args.get(0).context("value")?;
            tdvm.memory.insert(
                "xXx_return_xXx".into(),
                Var {
                    value: value.clone(),
                },
            );

            Ok(Value::Void)
        }),
    }
}
