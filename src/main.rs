use std::{env, fs, path::PathBuf};

use json_parser;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let json = json_parser::JsonParser::new();
    match fs::canonicalize(&PathBuf::from(&args[1])) {
        Ok(filepath) => match fs::read_to_string(&filepath) {
            Ok(contents) => {
                println!("{:#?}", contents);
                println!("Tokens: {:#?}", json.tokenize(contents));
            }
            Err(e) => {
                eprintln!("{}", e.to_string());
            }
        },
        Err(e) => {
            eprintln!("{}", e.to_string())
        }
    };
}
