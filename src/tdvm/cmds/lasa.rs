use anyhow::Context;

use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest, Command},
        types::Type,
        value::Extract,
        var::Var,
    },
};

pub fn lasa() -> Command {
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
    }
}
