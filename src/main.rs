mod args;

use std::{collections::HashMap, fs::read_to_string};

use clap::Parser;
use regex::Regex;
use chrono::prelude::Local;
use hayagriva::{Entry, Library, io::{from_yaml_str, from_biblatex_str}};

use crate::args::Configuration;

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

fn construct_string(citation: HashMap<String, &Entry>, config: Configuration) -> String {
	let yaml = serde_saphyr::to_string(&citation).unwrap();

	let topics = if !config.topic.is_empty() {
		format!("topics: \n  - {}", config.topic.join("\n  - "))
	} else {
		String::new()
	};
	
	format!(r#"
{yaml}  added_at: {}
  relevance: {}
  read: false
  {}
"#, 
		Local::now().to_string(),
		config.relevance.unwrap_or("2".to_string()),
		topics
	)
}

fn main() {
	let config = Configuration::parse();
    
    let hg = from_biblatex_str(&read_to_string(&config.bib_path).unwrap()).unwrap();
    let entry = hg.nth(0).unwrap();

    let mut citation = HashMap::new();
    let mut id = get_id(entry);
    
    config.hg_path.as_ref().inspect(|hg_path| {
	    let existing_hg = from_yaml_str(&read_to_string(hg_path).unwrap()).unwrap();
		let test = |hg: &Library, id: &String | { hg.iter().any(|entry| { entry.key() == id }) };

		let mut suffix = 0;
		let original_id = id.clone();
		while test(&existing_hg, &id) { 
			eprint!("WARNING: duplicate ID found: `{}`. ", id);
			suffix += 1; 
			id = format!("{}_{}", original_id, suffix);
			eprintln!("Replacing with: `{}`", id);
		}
    });

    citation.insert(id, entry);
    println!("{}", construct_string(citation, config));
}
