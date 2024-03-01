use anyhow::Context;

use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest, Command, Inner},
        types::Type,
        value::Extract,
        var::Var,
    },
};

pub fn testlasa() -> Command {
    Command {
        name: "testlasa".into(),
        requested_args: args!(name: Str, value: Any),
        inner: Inner::Rusty(|args, tdvm| {
            let name = args.get(0).context("name")?.extract_str()?;
            let value = args.get(1).context("value")?;
            tdvm.test_memory.insert(
                name,
                Var {
                    value: value.clone(),
                },
            );

            Ok(value.clone())
        }),
    }
}
