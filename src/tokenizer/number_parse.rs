use crate::tdvm::value::Value;

use super::token::Token;

pub(crate) fn number_parse(input: &str, cursor: &usize) -> Option<(Token, usize)> {
    let first = input.chars().nth(*cursor)?;

    let mut inner: Vec<char> = vec![first];
    let mut subcursor = *cursor + 1;

    loop {
        let c = input.chars().nth(subcursor);
        if let None = c {
            break;
        }

        let c = c?;
        if c != '.' && !c.is_numeric() {
            break;
        }
        inner.push(c);
        subcursor += 1;
    }

    let s = String::from_iter(inner);
    let n: f64 = s.parse().ok()?;

    return Some((Token::Value(Value::Num(n)), subcursor));
}
