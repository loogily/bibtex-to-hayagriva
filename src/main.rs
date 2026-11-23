use std::{collections::HashMap, process::exit};

use regex::Regex;

fn get_input(arg: &String) -> biblatex::Bibliography {
    let content = match std::fs::read_to_string(arg) {
        Ok(c) => { c },
        Err(_) => { arg.clone() },
    };

    match biblatex::Bibliography::parse(&content) {
        Err(_) => { eprintln!("Invalid bibTeX input."); exit(-1); },
        Ok(b) => { b }
    }
}

fn get_id(hg: &hayagriva::Entry) -> String {
    let title = hg.title().unwrap().to_string();
    let author = hg.authors().unwrap()[0].name.clone().to_lowercase();

    let title = title.replace(" ", "_");
    let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    let title = re.replace_all(title.as_str(), "_").to_string();
    let re = Regex::new(r"_+").unwrap();
    let title = re.replace_all(title.as_str(), "_").to_string();
    let re = Regex::new(r"[^_]*_([^_]*_){2}").unwrap();
    let title = match re.find(title.as_str()) {
        Some(t) => t.as_str().to_lowercase()[0..(t.len()-1)].to_string(),
        None    => title.to_lowercase(),
    };

    std::format!("{title}_{author}")
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Too few arguments");
        exit(-1);
    }
    
    let bibtex_in = get_input(&args[1]).into_vec();

    let hg: hayagriva::Entry = match (&bibtex_in[0]).try_into() {
        Err(_) => { eprintln!("Failed to convert."); exit(-1); },
        Ok(hg) => { hg },
    };

    let mut citation = HashMap::new();
    citation.insert(get_id(&hg), hg);
    let yaml = serde_saphyr::to_string(&citation).unwrap();
    print!("{yaml}");

    let mut topic = "null".to_string(); 
    if args.len() >= 3 { topic = args[2].clone(); }
    println!("  topic: {}", topic);

    let mut relevance: String = "3".to_string();
    if args.len() >= 4 { relevance = args[3].clone(); }
    println!("  relevance: {}", relevance);
    
    println!("  read: false");

    println!("");
}
