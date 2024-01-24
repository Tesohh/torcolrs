use crate::{tdvm::tdvm::Tdvm, tokenizer::tokenizer::tokenize};

mod tdvm;
mod tokenizer;

fn main() {
    let tdvm = Tdvm::default();
    // dbg!(tdvm);
    dbg!(tokenize(r#"stampa (vera)"#, &tdvm));
}
