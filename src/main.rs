use crate::tokenizer::tokenizer::tokenize;

pub mod tokenizer;

fn main() {
    dbg!(tokenize(r#"{{"hi"}}"#));
}
