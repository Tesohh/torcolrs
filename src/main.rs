use crate::tdvm::tdvm::Tdvm;

mod tdvm;
mod tokenizer;

fn main() -> anyhow::Result<()> {
    let mut tdvm = Tdvm::default();
    // dbg!(tokenize(r#"(stampa 5 4.2 ") )"#, &tdvm));

    tdvm.input = r#"stampa {
    stampa "gozzo"
    lasa "x" 7
    }
    stampa "cissyyyy"
    "#
    .into();

    dbg!(tdvm.run())

    // dbg!(tdvm);
    // dbg!(tokenize(
    //     r#"stampa (vera) feiowjfoijw "GIURAA" 12 34 69.420 # ciaoo "morgen""#,
    //     &tdvm
    // ));
}
