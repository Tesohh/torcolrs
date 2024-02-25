use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest, Command},
        types::Type,
        value::Value,
    },
};

fn cmd() -> Command {
    Command {
        name: "cmd".into(),
        requested_args: args!(name: Str, block: Block),
        inner: |args, tdvm| Ok(Value::Void),
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
//   Nothing much, just add cases to verify_args
//
// RUN STAGE
//
