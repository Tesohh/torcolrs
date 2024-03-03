use anyhow::{bail, Context};

use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest, Command, Inner},
        types::Type,
        value::{Extract, Value},
    },
};

pub fn stampa() -> Command {
    Command {
        name: "stampa".into(),
        requested_args: args!(val: Any),
        inner: Inner::Rusty(|args, _tdvm| {
            let value = args.get(0).context("first arg is empty")?;

            match value {
                Value::Ident(_) => bail!(
                    "cannot print identifier (maybe {} isn't a variable?)",
                    value.extract_str()?
                ),
                _ => {}
            }

            println!("{}", value);
            Ok(Value::Void)
        }),
    }
}
