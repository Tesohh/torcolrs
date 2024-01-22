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

/// run for every line!
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

    return tokens;
}

#[cfg(test)]
mod tests {
    use crate::{tokenizer::token::Token, *};

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize(r#"())) {"#),
            vec![
                Token::ParOpen,
                Token::ParClose,
                Token::ParClose,
                Token::ParClose,
                Token::SquirlyOpen,
            ]
        );

        assert_eq!(
            tokenize(r#"("heyy")"#),
            vec![Token::ParOpen, Token::Str("heyy".into()), Token::ParClose,]
        );
    }
}
