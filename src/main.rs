use crate::{tdvm::tdvm::Tdvm, tokenizer::tokenizer::tokenize};

mod tdvm;
mod tokenizer;

fn main() -> anyhow::Result<()> {
    let mut tdvm = Tdvm::default();
    // dbg!(tokenize(r#"(stampa 5 4.2  "#, &tdvm));

    tdvm.input = r#"stampa ( ciao (stampa) (stampa) )
    stampa"#
        .into();

    let res = tdvm.run()?;

    // dbg!(tdvm);
    // dbg!(tokenize(
    //     r#"stampa (vera) feiowjfoijw "GIURAA" 12 34 69.420 # ciaoo "morgen""#,
    //     &tdvm
    // ));
    Ok(())
}
