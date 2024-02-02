#![allow(unused)] // TODO: remove this
use std::collections::HashMap;

use anyhow::{anyhow, bail, Context, Ok};

use crate::tokenizer::{
    token::{Token, Tokens},
    tokenizer::tokenize,
};

use super::{cmds::commands, command::Command, value::Value, var::Var};

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
    pub fn run_cmd(&mut self, tokens: &mut Tokens) -> anyhow::Result<Value> {
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

    pub fn run_scoped(&mut self, input: String) -> anyhow::Result<()> {
        let old_input = self.input.clone();
        let old_mem = self.memory.clone();
        let old_cursor = self.linecursor.clone();

        self.input = input;
        self.linecursor = 0;
        self.run()?;

        self.input = old_input;
        self.linecursor = old_cursor;
        let mut remove: Vec<String> = vec![];
        for (k, v) in &self.memory {
            if old_mem.contains_key(k) {
                continue;
            }
            remove.push(k.into());
        }

        for v in remove {
            self.memory.remove(&v).context("cannot remove key")?;
        }

        Ok(())
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

            // filter out Comments
            let mut tokens: Vec<_> = tokenize(line, self)
                .into_iter()
                .filter(|tok| match tok {
                    Token::Comment(_) => false,
                    _ => true,
                })
                .collect();

            let last = tokens.last_mut().context("out of bounds")?;
            if last == &mut Token::SquirlyOpen {
                let mut inner: String = "".into();
                let mut count = 0;
                let mut nesting_level = 1;
                let mut delete_targets: Vec<usize> = vec![];

                for (i, l) in self.input.lines().skip(self.linecursor + 1).enumerate() {
                    for c in l.chars() {
                        if c == '{' {
                            nesting_level += 1;
                        } else if c == '}' {
                            nesting_level -= 1;
                            delete_targets.push(i);
                            if nesting_level == 0 {
                                count = i + 1;
                                break;
                            }
                        }
                    }

                    if nesting_level == 0 {
                        break;
                    }

                    inner += &String::from(l);
                    inner += "\n";
                    delete_targets.push(i);
                    // it works correctly: we just need to take the } out

                    // maybe it's becasue it's a raw string it
                    // doesn't recognize \n as a newline (everything gets treated as one line)
                    // or maybe collect puts them in a whole
                    // line because it's trying to make a
                    // String, maybe if we do Vec<String>
                }

                *last = Token::Value(Value::Block(inner));
                self.input = self
                    .input
                    .lines()
                    .enumerate()
                    // .filter(|&(i, _)| i < self.linecursor + 1 || i > self.linecursor + count)
                    .filter(|&(i, _)| !delete_targets.contains(&i))
                    .map(|(_, l)| String::from(l) + "\n")
                    .collect();
            }

            // check that the first token is a command
            // dbg!(&self.linecursor, &tokens);
            let first = tokens.get(0).context("first token doesn't exist")?;
            match first {
                Token::Cmd(_) => (),
                _ => return self.gen_err(anyhow!("first token is not a Cmd {:?}", first)),
            }

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
