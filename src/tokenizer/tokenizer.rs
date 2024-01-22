use super::token::{Token, Tokens};

fn try_identify(input: char) -> Option<Token> {
    match input {
        '(' => Some(Token::ParOpen),
        ')' => Some(Token::ParClose),
        '{' => Some(Token::SquirlyOpen),
        '}' => Some(Token::SquirlyClose),
        _ => None,
    }
}

fn block(input: &str, cursor: &usize) -> Option<(Token, usize)> {
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

pub fn tokenize(input: &str) -> Tokens {
    let mut tokens: Tokens = vec![];
    let mut cursor = 0usize;

    while cursor < input.len() {
        let tok = try_identify(input.chars().nth(cursor).expect("out of bounds"));
        if let Some(v) = tok {
            tokens.push(v);
            cursor += 1;
            continue;
        }

        let tok = block(input, &cursor);
        if let Some(v) = tok {
            tokens.push(v.0);
            cursor = v.1;
            continue;
        }
        cursor += 1;
    }

    // for i in input.chars() {
    //     // try single char token
    //     let tok = try_identify(i);
    //     if let Some(v) = tok {
    //         tokens.push(v);
    //         cursor += 1;
    //         continue;
    //     }
    //
    //     // we are dealing with a string
    //     if i == '"' {}
    // }

    return tokens;
}
