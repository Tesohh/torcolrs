use crate::tdvm::{tdvm::Tdvm, value::Value};

use super::{token::Token, tokenizer::try_identify};

fn identify_word(input: String, tdvm: &Tdvm) -> Token {
    let cmd_names: Vec<_> = tdvm.commands.iter().map(|c| &c.name).collect();
    if cmd_names.contains(&&input) {
        return Token::Cmd(input);
    }

    if tdvm.types.contains(&input) {
        return Token::Type(input.into());
    }

    if tdvm.memory.contains_key(&input) {
        return Token::Var(input);
    }

    match input.as_str() {
        "vera" => Token::Value(Value::Bool(true)),
        "faus" => Token::Value(Value::Bool(false)),

        &_ => Token::Ident(input),
    }
}

pub(crate) fn parse_word(input: &str, cursor: &usize, tdvm: &Tdvm) -> Option<(Token, usize)> {
    let first = input.chars().nth(*cursor)?;

    let mut inner: Vec<char> = vec![first];
    let mut subcursor = *cursor + 1;

    loop {
        let c = input.chars().nth(subcursor);
        if let None = c {
            break;
        }

        let c = c?;
        if try_identify(c).is_some() {
            break;
        }
        if c.is_whitespace() {
            subcursor += 1;
            break;
        }
        inner.push(c);
        subcursor += 1;
    }

    let s = String::from_iter(inner);
    let tok = identify_word(s, &tdvm);

    Some((tok, subcursor))
}
