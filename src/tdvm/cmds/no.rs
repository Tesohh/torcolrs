use anyhow::Context;

use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest, Command, Inner},
        types::Type,
        value::{Extract, Value},
    },
};

pub fn no() -> Command {
    Command {
        name: "no".into(),
        requested_args: args!(b: Bool),
        inner: Inner::Rusty(|args, _| {
            let b = args
                .get(0)
                .context("IndexArrayOutOfBoundsException")?
                .extract_bool()?;
            Ok(Value::Bool(!b))
        }),
    }
}
