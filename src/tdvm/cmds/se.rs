use anyhow::Context;

use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest, Command},
        types::Type,
        value::{Extract, Value},
    },
};

pub fn se() -> Command {
    Command {
        name: "se".into(),
        requested_args: args!(condition: Bool, block: Block),
        inner: |args, tdvm| {
            let cond = args.get(0).context("cond")?.extract_bool()?;
            let block = args.get(1).context("block")?.extract_block()?;
            if !cond {
                return Ok(Value::Void);
            }

            tdvm.run_scoped(block)?;

            Ok(Value::Void)
        },
    }
}
