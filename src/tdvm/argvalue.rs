use std::collections::HashMap;

use anyhow::anyhow;

use super::{types::Type, value::Value};

pub struct ArgValue {
    name: String,
    value: Value,
    typ: Type,
}

pub struct ArgValues(Vec<ArgValue>);

// is this really necessary? mostly it's for error logging,
// so you could just implement a trait on
// Option<ArgValue> that is returned from .get(0)
// which adds a custom .context()
// or would that conflict with anyhow?
// maybe it's better to just do this..
impl ArgValues {
    pub fn get(&self, key: String) -> anyhow::Result<Value> {
        for i in &self.0 {
            if i.name == key {
                return Ok(i.value.clone());
            }
        }

        return Err(anyhow!("argument {} not found", key));
    }
}
