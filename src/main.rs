use std::fs::read;

use crate::tdvm::tdvm::Tdvm;

mod tdvm;
mod tokenizer;

fn main() -> anyhow::Result<()> {
    let mut tdvm = Tdvm::default();
    // dbg!(tokenize(r#"(stampa 5 4.2 ") )"#, &tdvm));

    tdvm.input = String::from_utf8(read("tr/cissy.tr")?)?;

    // this is a problem: run doesnt recognize subblocks as a block but as a SquirlyOpen and thats it.
    // You needa do a parse_block function that takes in Vec<Tokens> that does the whole put in a
    // Value::Block business, and remove shit from the tdvm.input
    // also you need to do that chatgpt thing to recognize the correct bracket to end the block
    // instead of the first one you see
    //
    // also for scoping variables you can also do a run_scoped function that runs the block,
    // then checks tdvm.memory and delete all new keys
    //
    // OR
    // scrap tokenizing inner blocks:
    // instead in Block save just the string
    // then implement the run_scoped function which just passes the Block's string and increments
    // linecursor accordingly.
    // ggez clap

    tdvm.run()

    // dbg!(tdvm);
    // dbg!(tokenize(
    //     r#"stampa (vera) feiowjfoijw "GIURAA" 12 34 69.420 # ciaoo "morgen""#,
    //     &tdvm
    // ));
}
