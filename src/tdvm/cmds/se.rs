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

#[cfg(test)]
mod tests {
    use crate::tdvm::tdvm::Tdvm;

    #[test]
    fn test_scope_execution() {
        let mut tdvm = Tdvm::default();
        tdvm.input = r#"se vera {
            lasa "x" 1
        }"#
        .into();
        tdvm.run().unwrap();

        assert!(tdvm.memory.is_empty());
    }

    #[test]
    fn test_se_skipping() {
        let mut tdvm = Tdvm::default();
        tdvm.input = r#"se faus {
            testlasa "x" 1
        }"#
        .into();
        tdvm.run().unwrap();

        assert!(tdvm.test_memory.is_empty());
    }
}
