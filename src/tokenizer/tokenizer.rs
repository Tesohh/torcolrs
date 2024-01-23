use super::token::{Token, Tokens};
use super::{blocks, number_parse};

fn try_identify(input: char) -> Option<Token> {
    match input {
        '(' => Some(Token::ParOpen),
        ')' => Some(Token::ParClose),
        '{' => Some(Token::SquirlyOpen),
        '}' => Some(Token::SquirlyClose),
        _ => None,
    }
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

        let tok = blocks::block(input, &cursor);
        if let Some(v) = tok {
            tokens.push(v.0);
            cursor = v.1;
            continue;
        }

        let tok = number_parse::number_parse(input, &cursor);
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

        // FIX: unfinisheed string should parse as string until the end
        // this could be implemented similarly to how number_parse works
        assert_eq!(
            tokenize(r#"("morgen)"#),
            vec![Token::ParOpen, Token::ParClose]
        );

        assert_eq!(
            tokenize(r#"("heyy 12 34.34")"#),
            vec![
                Token::ParOpen,
                Token::Str("heyy 12 34.34".into()),
                Token::ParClose,
            ]
        );

        assert_eq!(
            tokenize(r#"12 32 43.34"#),
            vec![Token::Num(12.0), Token::Num(32.0), Token::Num(43.34)]
        );
    }
}
