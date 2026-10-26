use std::process::exit;

fn main() {
    let args = dbg!(std::env::args().collect::<Vec<String>>());

    if args.len() < 2 {
        eprintln!("Too few arguments");
        exit(-1);
    }

    let bibtex_in = &args[1];
}
