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

// Ident
// PARSING STAGE
// when you see some plain text, it may be a Var, Ident or a Type
// x add tokens for Ident and Type
// x add values for Ident and Type
// x add types for  Ident and Type
//
// PRERUN STAGE
// x  Nothing much, just add cases to verify_args
//
// RUN STAGE
//    create the command and add it to the tdvm
//    do something to parse custom command argument types and such?
//      perhaps with a command that takes a name and a type, and makes a string with all data that
//      can be "parsed" from cmd?
