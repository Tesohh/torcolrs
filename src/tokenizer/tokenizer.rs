use crate::tdvm::tdvm::Tdvm;

use super::parse_word::parse_word;
use super::token::{Token, Tokens};
use super::{blocks, number_parse};

pub fn try_identify(input: char) -> Option<Token> {
    match input {
        '(' => Some(Token::ParOpen),
        ')' => Some(Token::ParClose),
        '{' => Some(Token::SquirlyOpen),
        '}' => Some(Token::SquirlyClose),
        _ => None,
    }
}

/// run for every line!
pub fn tokenize(input: &str, tdvm: &Tdvm) -> Tokens {
    let mut tokens: Tokens = vec![];
    let mut cursor = 0usize;

    // if input.starts_with("#") {
    //     return vec![Token::Comment(input.into())];
    // }

    while cursor < input.len() {
        if input
            .chars()
            .nth(cursor)
            .expect("out of bounds")
            .is_whitespace()
        {
            cursor += 1;
            continue;
        }

        if input.chars().nth(cursor).expect("out of bounds") == '#' {
            let comment = input.get(cursor..).expect("out of bounds");
            tokens.push(Token::Comment(comment.into()));
            break;
        }

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

        let tok = parse_word(input, &cursor, &tdvm);
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
    use crate::{
        tdvm::tdvm::{Tdvm, Value},
        tokenizer::token::Token,
        *,
    };

    #[test]
    fn test_tokenize() {
        let mut tdvm = Tdvm::default();
        tdvm.memory.insert("ciord".into(), Value::Num(34.0));
        assert_eq!(
            tokenize(r#"())) {"#, &tdvm),
            vec![
                Token::ParOpen,
                Token::ParClose,
                Token::ParClose,
                Token::ParClose,
                Token::SquirlyOpen,
            ]
        );

        assert_eq!(
            tokenize(r#"("heyy")"#, &tdvm),
            vec![
                Token::ParOpen,
                Token::Value(Value::Str("heyy".into())),
                Token::ParClose,
            ]
        );

        assert_eq!(
            tokenize(r#"("morgen)"#, &tdvm),
            vec![Token::ParOpen, Token::Value(Value::Str("morgen)".into()))]
        );

        assert_eq!(
            tokenize(r#"("heyy 12 34.34")"#, &tdvm),
            vec![
                Token::ParOpen,
                Token::Value(Value::Str("heyy 12 34.34".into())),
                Token::ParClose,
            ]
        );

        assert_eq!(
            tokenize(r#"12 32 43.34"#, &tdvm),
            vec![
                Token::Value(Value::Num(12.0)),
                Token::Value(Value::Num(32.0)),
                Token::Value(Value::Num(43.34))
            ]
        );

        assert_eq!(
            tokenize(r#"() # gozzo 12"#, &tdvm),
            vec![
                Token::ParOpen,
                Token::ParClose,
                Token::Comment("# gozzo 12".into())
            ]
        );

        assert_eq!(
            tokenize(r#"stampa vera (faus) GOZZO"#, &tdvm),
            vec![
                Token::Cmd("stampa".into()),
                Token::Value(Value::Bool(true)),
                Token::ParOpen,
                Token::Value(Value::Bool(false)),
                Token::ParClose,
                Token::Unknown("GOZZO".into()),
            ]
        );

        assert_eq!(
            tokenize(r#"Num Str Bool ciord fassa"#, &tdvm),
            vec![
                Token::Type("Num".into()),
                Token::Type("Str".into()),
                Token::Type("Bool".into()),
                Token::Var("ciord".into()),
                Token::Unknown("fassa".into()),
            ]
        );
    }
}
