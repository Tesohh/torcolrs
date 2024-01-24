#![allow(unused)] // TODO: remove this
use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<Value>),
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
