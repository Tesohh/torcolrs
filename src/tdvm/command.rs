use std::any::Any;

use anyhow::{bail, Context};

use crate::tokenizer::token::{Token, Tokens};

use super::{tdvm::Tdvm, types::Type, value::Value};

#[derive(Debug)]
pub struct Arg {
    pub name: String,
    pub expected: Type,
}

#[derive(Debug)]
pub enum ArgsRequest {
    Limited(Vec<Arg>),
    LastIsInfinite(Vec<Arg>),
    Void,
}

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub requested_args: ArgsRequest,
    pub inner: fn(args: Vec<Value>, tdvm: &Tdvm) -> anyhow::Result<Value>,
}

impl Command {
    fn verify_args(&self, tokens: &Tokens) -> anyhow::Result<()> {
        match &self.requested_args {
            ArgsRequest::Limited(args) => {
                if args.len() != tokens.len() {
                    bail!(
                        "command {} expected {} args, got {}",
                        self.name,
                        args.len(),
                        tokens.len()
                    )
                }

                for (i, arg) in args.iter().enumerate() {
                    let tok = tokens.get(i).context("out of bounds")?;

                    let val = match tok {
                        Token::Var(_) => todo!("will get the var from memory"),
                        Token::Value(val) => val,
                        _ => bail!(
                            "command {} on arg {}: got unexpected token",
                            self.name,
                            arg.name
                        ),
                    };

                    let t: Type = val.into();
                    if arg.expected != t {
                        bail!(
                            "command {} on arg {}: got {}, expected {}",
                            self.name,
                            arg.name,
                            t,
                            arg.expected
                        )
                    }
                }
            }

            ArgsRequest::LastIsInfinite(_) => todo!(),
            ArgsRequest::Void => {}
        };

        Ok(())
    }

    pub fn run(&self, tokens: Tokens, tdvm: &Tdvm) -> anyhow::Result<Value> {
        // check if arguments are correct
        self.verify_args(&tokens)?;

        let values: Vec<_> = tokens
            .into_iter()
            .filter_map(|tok| match tok {
                Token::Value(val) => Some(val),
                Token::Var(_) => todo!("will get the var from memory"),
                // _ => bail!("command {}: got unexpected token", self.name),
                // at this point they should all be values right?
                _ => None,
            })
            .collect();

        return (self.inner)(values, tdvm);
    }
}
