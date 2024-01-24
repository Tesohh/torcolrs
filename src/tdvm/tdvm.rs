#![allow(unused)] // TODO: remove this
use std::collections::HashMap;

use anyhow::{anyhow, Context};

use crate::tokenizer::{token::Token, tokenizer::tokenize};

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<Value>),
    // Arg((String, Type))
}

#[derive(Debug)]
pub struct Tdvm {
    pub input: String,
    linecursor: usize,
    pub commands: Vec<String>, // TEMP: will change to a hashmap probably, which takes in an array of tokens as args??
    pub types: Vec<String>,
    pub memory: HashMap<String, Value>,
}

impl Default for Tdvm {
    fn default() -> Self {
        Self {
            input: Default::default(),
            linecursor: Default::default(),
            commands: vec!["stampa".into(), "lasa".into()],
            types: vec!["Num".into(), "Str".into(), "Bool".into()],
            memory: Default::default(),
        }
    }
}

impl Tdvm {
    fn gen_err(&self, err: anyhow::Error) -> Result<(), anyhow::Error> {
        return Err(anyhow!("[tdvm: line {}] {}", self.linecursor + 1, err));
    }
    pub fn run(&mut self) -> Result<(), anyhow::Error> {
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

            let tokens = tokenize(line, self);

            // check that the first token is a command
            match tokens.get(0).context("first token doesn't exist")? {
                Token::Cmd(_) => (),
                _ => return self.gen_err(anyhow!("first token is not a Cmd")),
            }

            // check that parentheses are closed
            let open_par_count = tokens.iter().filter(|tok| tok == &&Token::ParOpen).count();
            let close_par_count = tokens.iter().filter(|tok| tok == &&Token::ParClose).count();
            if open_par_count != close_par_count {
                return self.gen_err(anyhow!(
                    "open parentheses count is different to the closed parenthesis count"
                ));
            }

            // parse all subcommands
            // (try to) execute them
            // if error, ABORT EVERYTHING and panic
            // replace the subcommand with the return value
            // execute the main command
            self.linecursor += 1;
        }

        return Ok(());
    }
}
