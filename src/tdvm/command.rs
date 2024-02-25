use anyhow::{bail, Context};

use crate::tokenizer::token::{Token, Tokens};

use super::{tdvm::Tdvm, types::Type, value::Value};

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub expected: Type,
}

#[macro_export]
macro_rules! args {
    ($($name:ident : $ty:ident),*) => {
        {
            ArgsRequest::Limited(vec![
                $(
                    Arg {
                        name: stringify!($name).to_string(),
                        expected: Type::$ty,
                    },
                )*
            ])
        }
    };
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum ArgsRequest {
    Limited(Vec<Arg>),
    // LastIsInfinite(Vec<Arg>),
    Void,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub requested_args: ArgsRequest,
    pub inner: fn(args: Vec<Value>, tdvm: &mut Tdvm) -> anyhow::Result<Value>,
}

impl Command {
    fn verify_args(&self, tokens: &Tokens, tdvm: &mut Tdvm) -> anyhow::Result<()> {
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

                // check type correctness
                for (i, arg) in args.iter().enumerate() {
                    if arg.expected == Type::Any {
                        continue;
                    }

                    let mut badbadbadbad_variable_that_shouldnt_exist: Value = Value::Void;
                    let mut badbadbadbad_variable_that_shouldnt_exist_v2: Value = Value::Void;

                    let tok = tokens.get(i).context("out of bounds")?;

                    let val = match tok {
                        Token::Var(k) => {
                            &tdvm.memory.get(k).context("variable doesnt exist")?.value
                        }
                        Token::Value(v) => v,
                        Token::Ident(v) => {
                            badbadbadbad_variable_that_shouldnt_exist = Value::Ident(v.to_string());
                            &badbadbadbad_variable_that_shouldnt_exist
                        }
                        Token::Type(v) => {
                            badbadbadbad_variable_that_shouldnt_exist_v2 = Value::Type(v.clone());
                            &badbadbadbad_variable_that_shouldnt_exist_v2
                        }
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

            // ArgsRequest::LastIsInfinite(_) => todo!(),
            ArgsRequest::Void => {}
        };

        Ok(())
    }

    pub fn run(&self, tokens: Tokens, tdvm: &mut Tdvm) -> anyhow::Result<Value> {
        // check if arguments are correct
        let mut tokens = tokens;
        tokens.remove(0);
        self.verify_args(&tokens, tdvm)?;

        let values: Vec<_> = tokens
            .into_iter()
            .filter_map(|tok| match tok {
                Token::Value(val) => Some(val),
                Token::Var(key) => {
                    let var = tdvm.memory.get(&key).cloned();
                    match var {
                        Some(v) => Some(v.value),
                        None => None,
                    }
                }
                Token::Type(val) => Some(Value::Type(val)),
                Token::Ident(val) => Some(Value::Ident(val)),
                // _ => bail!("command {}: got unexpected token", self.name),
                // at this point they should all be values right?
                _ => None,
            })
            .collect();

        return (self.inner)(values, tdvm);
    }
}
