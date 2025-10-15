use snippets::{
    parse_paras::parse_para,
    snip::Snippets,
    SnipTag,
    error::MyError,
};

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> Result<(), MyError> {
    // parse paras
    let paras = parse_para()?;

    if paras.show_tags {
        println!("supported tags: {}", SnipTag::supported_tags());
        Ok(())
    } else {
        // init SnipType
        let my_snippets = Snippets::new(&paras.files, paras.model.clone())?;

        // run
        if paras.summary.is_empty() {
            my_snippets.get(paras)
        } else {
            my_snippets.print_summary(&paras.summary)
        }
    }
}
