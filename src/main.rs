use std::{collections::HashMap, hash::{BuildHasher, Hash}, io::BufWriter, iter::Map, process::exit};

use bibtex_parser::Library;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Too few arguments");
        exit(-1);
    }

    let bibtex_in = &args[1];

    let library = Library::parse(bibtex_in);
    let library = match library {
        Err(_) => { eprintln!("Invalid bibTeX input."); exit(-1); },
        Ok(l) => { l }
    };

    if library.entries().len() > 1 {
        eprintln!("Too many entries. Only include one.");
        exit(-1);
    }

    let entry = &library.entries()[0];

    let mut fields: HashMap<String, String> = HashMap::new();
    fields.insert("type".to_string(), entry.entry_type().to_string());
    
    for field in entry.fields() {
        fields.insert(field.name.to_string().to_lowercase(), field.value.to_plain_string());
    }

    let mut yaml: HashMap<String, HashMap<String, String>> = HashMap::new();
    yaml.insert(entry.key().to_string(), fields);
    yaml.remove("unique-id");
    let yaml_str = serde_saphyr::to_string(&yaml).unwrap();
    println!("\n\n{yaml_str}");
}
