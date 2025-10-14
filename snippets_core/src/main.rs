use snippets::{
    parse_paras::parse_para,
    snip::Snippets,
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

    // init SnipType
    let my_snippets = Snippets::new(&paras.files, paras.model.clone())?;

    // run
    if paras.summary.is_empty() {
        my_snippets.get(paras)
    } else {
        my_snippets.print_summary(&paras.summary)
    }
}
