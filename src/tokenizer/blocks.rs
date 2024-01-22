use super::token::Token;

pub(crate) fn block(input: &str, cursor: &usize) -> Option<(Token, usize)> {
    let first = input.chars().nth(*cursor)?;

    if !vec!['"'].contains(&first) {
        return None;
    }

    let mut inner: Vec<char> = vec![];
    let mut subcursor = *cursor + 1;

    loop {
        let c = input.chars().nth(subcursor)?;
        if c == first {
            break;
        }
        inner.push(c);
        subcursor += 1;
    }

    return match &first {
        '"' => Some((Token::Str(String::from_iter(inner)), subcursor)),
        _ => None,
    };
}

// pub static BLOCKS: HashMap<&str, &str> = HashMap::from([("(", ")"), ("{", "}")]);
