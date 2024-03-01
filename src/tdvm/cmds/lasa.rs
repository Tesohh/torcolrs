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

pub fn lasa() -> Command {
    Command {
        name: "lasa".into(),
        requested_args: args!(name: Str, value: Any),
        inner: Inner::Rusty(|args, tdvm| {
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
        }),
    }
}
#[cfg(test)]
mod tests {
    use crate::tdvm::{tdvm::Tdvm, value::Value};

    #[test]
    fn lasa_test() {
        let mut tdvm = Tdvm::default();
        tdvm.input = r#"lasa "x" 5
        lasa "y" "cissy""#
            .into();
        tdvm.run().unwrap();

        assert_eq!(tdvm.memory.get("x").unwrap().value, Value::Num(5.0));
        assert_eq!(
            tdvm.memory.get("y").unwrap().value,
            Value::Str("cissy".into())
        );
    }
}
