use crate::tdvm::tdvm::Tdvm;

use super::{token::Token, tokenizer::try_identify};

fn identify_word(input: String, tdvm: &Tdvm) -> Token {
    if tdvm.commands.contains(&input) {
        return Token::Cmd(input);
    }

    if tdvm.types.contains(&input) {
        return Token::Type(input);
    }

    if tdvm.memory.contains_key(&input) {
        return Token::Var(input);
    }

    match input.as_str() {
        "vera" => Token::Bool(true),
        "faus" => Token::Bool(false),

        &_ => Token::Unknown(input),
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
