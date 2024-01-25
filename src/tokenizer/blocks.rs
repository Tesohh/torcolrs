use std::collections::HashMap;

use crate::tdvm::tdvm::{Tdvm, Value};

use super::{token::Token, tokenizer::tokenize};

pub(crate) fn block(input: &str, cursor: &usize, tdvm: &Tdvm) -> Option<(Token, usize)> {
    let first = input.chars().nth(*cursor)?;

    let first_last: HashMap<_, _> = HashMap::from([('"', '"'), ('(', ')')]);
    let expected_last = *first_last.get(&first)?;

    if !vec!['"', '('].contains(&first) {
        return None;
    }

    let mut inner: Vec<char> = vec![];
    let mut subcursor = *cursor + 1;
    let mut bracket_count = 1;

    loop {
        let c = input.chars().nth(subcursor);
        if c == None {
            break;
        }
        if c == Some(expected_last) {
            bracket_count -= 1;
            if bracket_count == 0 {
                subcursor += 1;
                break;
            }
        } else if c == Some(first) {
            bracket_count += 1;
        }
        let c = c?;
        inner.push(c);
        subcursor += 1;
    }

    let s = String::from_iter(inner);

    let tokens = tokenize(s.as_str(), &tdvm);

    return match &first {
        '"' => Some((Token::Value(Value::Str(s)), subcursor)),
        '(' => Some((Token::Sub(tokens), subcursor)),
        _ => None,
    };
}

// pub static BLOCKS: HashMap<&str, &str> = HashMap::from([("(", ")"), ("{", "}")]);
