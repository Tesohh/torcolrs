use crate::tdvm::tdvm::Value;

use super::token::Token;

pub(crate) fn block(input: &str, cursor: &usize) -> Option<(Token, usize)> {
    let first = input.chars().nth(*cursor)?;

    if !vec!['"'].contains(&first) {
        return None;
    }

    let mut inner: Vec<char> = vec![];
    let mut subcursor = *cursor + 1;

    loop {
        let c = input.chars().nth(subcursor);
        if c == None {
            break;
        }
        if c == Some(first) {
            subcursor += 1;
            break;
        }
        let c = c?;
        inner.push(c);
        subcursor += 1;
    }

    let s = String::from_iter(inner);

    return match &first {
        '"' => Some((Token::Value(Value::Str(s)), subcursor)),
        _ => None,
    };
}

// pub static BLOCKS: HashMap<&str, &str> = HashMap::from([("(", ")"), ("{", "}")]);
