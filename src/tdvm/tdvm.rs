#![allow(unused)] // TODO: remove this
use std::collections::HashMap;

use anyhow::{anyhow, Context, Ok};

use crate::tokenizer::{
    token::{Token, Tokens},
    tokenizer::tokenize,
};

use super::{command::Command, commands::commands, value::Value, var::Var};

#[derive(Debug)]
pub struct Tdvm {
    pub input: String,
    linecursor: usize,
    pub commands: Vec<Command>, // TEMP: will change to a hashmap probably, which takes in an array of tokens as args??
    pub types: Vec<String>,
    pub memory: HashMap<String, Var>,
}

impl Default for Tdvm {
    fn default() -> Self {
        Self {
            input: Default::default(),
            linecursor: Default::default(),
            commands: commands(),
            types: vec!["Num".into(), "Str".into(), "Bool".into()],
            memory: Default::default(),
        }
    }
}

impl Tdvm {
    fn gen_err(&self, err: anyhow::Error) -> anyhow::Result<()> {
        return Err(anyhow!("[tdvm: line {}] {}", self.linecursor + 1, err));
    }
    fn run_cmd(&mut self, tokens: &mut Tokens) -> anyhow::Result<Value> {
        // first run all subs
        // recursively...
        for i in 0..tokens.len() {
            if let Token::Sub(subtokens) = tokens.get_mut(i).context("out of bounds")? {
                let value = self.run_cmd(subtokens)?;
                tokens[i] = Token::Value(value)
            }
        }

        // then execute the current command...
        // find the current command

        let mut cmd: Option<Command> = None;
        if let Token::Cmd(name) = tokens.get(0).context("out of bounds")? {
            for c in &self.commands {
                if &c.name == name {
                    cmd = Some(c.clone())
                }
            }
        }

        let cmd = cmd.context("no command with that name was found")?;

        return cmd.run(tokens.to_vec(), self);
    }
    pub fn run(&mut self) -> anyhow::Result<()> {
        if self.input.is_empty() {
            return Err(anyhow!("input is empty"));
        }

        while self.linecursor < self.input.lines().count() {
            let line = self
                .input
                .lines()
                .nth(self.linecursor)
                .context("out of bounds")?;

            if line.trim().is_empty() {
                self.linecursor += 1;
                continue;
            }

            let mut tokens = tokenize(line, self);

            // check that the first token is a command
            match tokens.get(0).context("first token doesn't exist")? {
                Token::Cmd(_) => (),
                _ => return self.gen_err(anyhow!("first token is not a Cmd")),
            }

            // check that parentheses are closed
            // let open_par_count = tokens.iter().filter(|tok| tok == &&Token::ParOpen).count();
            // let close_par_count = tokens.iter().filter(|tok| tok == &&Token::ParClose).count();
            // if open_par_count != close_par_count {
            //     return self.gen_err(anyhow!(
            //         "open parentheses count is different to the closed parenthesis count"
            //     ));
            // }

            // (try to) execute subs
            self.run_cmd(&mut tokens)?;
            // if error, ABORT EVERYTHING and panic
            // replace the subcommand with the return value
            // execute the main command
            self.linecursor += 1;
        }

        return Ok(());
    }
}
