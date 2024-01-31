use anyhow::Context;

use crate::{
    args,
    tdvm::{
        command::{Arg, ArgsRequest, Command},
        types::Type,
        value::{Extract, Value},
    },
};

pub fn jonta() -> Command {
    Command {
        name: "jonta".into(),
        requested_args: args!(num1: Num, num2: Num),
        inner: |args, _tdvm| {
            let args = (args.get(0).context("num1")?, args.get(1).context("num2")?);
            let n1 = args.0.extract_num()?;
            let n2 = args.1.extract_num()?;
            Ok(Value::Num(n1 + n2))
        },
    }
}

pub fn sotra() -> Command {
    Command {
        name: "sotra".into(),
        requested_args: args!(num1: Num, num2: Num),
        inner: |args, _tdvm| {
            let args = (args.get(0).context("num1")?, args.get(1).context("num2")?);
            let n1 = args.0.extract_num()?;
            let n2 = args.1.extract_num()?;
            Ok(Value::Num(n1 - n2))
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::tdvm::tdvm::Tdvm;

    use super::*;

    #[test]
    fn test_jonta() {
        let mut tdvm = Tdvm::default();
        tdvm.input = r#"lasa "x" (jonta 5 2)"#.into();
        tdvm.run().unwrap();
        assert_eq!(tdvm.memory.get("x").unwrap().value, Value::Num(7.0))
    }

    #[test]
    fn test_error_jonta() {
        let mut tdvm = Tdvm::default();
        tdvm.input = r#"lasa "x" (jonta "cissy" 2)"#.into();
        let err = tdvm.run();
        assert!(!err.is_ok());
    }

    #[test]
    fn test_sotra() {
        let mut tdvm = Tdvm::default();
        tdvm.input = r#"lasa "x" (sotra 5 2)"#.into();
        tdvm.run().unwrap();
        assert_eq!(tdvm.memory.get("x").unwrap().value, Value::Num(3.0))
    }

    #[test]
    fn test_error_sotra() {
        let mut tdvm = Tdvm::default();
        tdvm.input = r#"lasa "x" (sotra "cissy" 2)"#.into();
        let err = tdvm.run();
        assert!(!err.is_ok());
    }
}
