use std::{collections::HashMap, process::exit};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Too few arguments");
        exit(-1);
    }

    let bibtex_in = match biblatex::Bibliography::parse(&args[1]) {
        Err(_) => { eprintln!("Invalid bibTeX input."); exit(-1); },
        Ok(b) => {b}
    }.into_vec();


    let hg: hayagriva::Entry = match (&bibtex_in[0]).try_into() {
        Err(_) => { eprintln!("Failed to convert."); exit(-1); },
        Ok(hg) => { hg },
    };

    
    let mut citation = HashMap::new();
    citation.insert(hg.key().to_string(), hg);
    let yaml = serde_saphyr::to_string(&citation).unwrap();
    print!("{yaml}");

    if args.len() < 3 { return; }
    println!("  topic: {}", &args[2]);
    if args.len() < 4 { return; }
    println!("  relevance: {}", &args[3]);
}
