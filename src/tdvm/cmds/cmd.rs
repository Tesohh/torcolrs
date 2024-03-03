use anyhow::Context;

use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest, Command, Inner},
        types::Type,
        value::{Extract, Value},
    },
};

pub fn cmd() -> Command {
    Command {
        name: "cmd".into(),
        requested_args: args!(name: Ident, block: Block),
        inner: Inner::Rusty(|args, tdvm| {
            let name = args.get(0).context("name")?.extract_str()?;
            let block = args.get(1).context("block")?.extract_block()?;
            let cmd = Command {
                name,
                requested_args: args!(),
                inner: Inner::Torcoly(block),
            };
            tdvm.commands.push(cmd);
            Ok(Value::Void)
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::tdvm::{tdvm::Tdvm, value::Value};

    #[test]
    fn test_cmd_saving() {
        let mut tdvm = Tdvm::default();
        tdvm.input = r#"
            cmd contacts {
                return "underground"
            }
            lasa "x" (contacts)
        "#
        .into();
        tdvm.run().unwrap();

        assert_eq!(
            tdvm.memory.get("x").unwrap().value,
            Value::Str("underground".into())
        )
    }
}
