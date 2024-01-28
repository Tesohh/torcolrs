use crate::tdvm::tdvm::Tdvm;

mod tdvm;
mod tokenizer;

fn main() -> anyhow::Result<()> {
    let mut tdvm = Tdvm::default();
    // dbg!(tokenize(r#"(stampa 5 4.2 ") )"#, &tdvm));

    tdvm.input = r#"lasa "x" 4
    stampa x
    lasa "s" "giura"
    lasa "x" (jonta s 5)
    stampa x"#
        .into();

    dbg!(tdvm.run())

    // dbg!(tdvm);
    // dbg!(tokenize(
    //     r#"stampa (vera) feiowjfoijw "GIURAA" 12 34 69.420 # ciaoo "morgen""#,
    //     &tdvm
    // ));
}
