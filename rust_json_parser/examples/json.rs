use json_parser;
use std::{env, fs, path::PathBuf};

fn main() {
    let mut args = env::args();
    let json = json_parser::JsonParser::new();

    // check for presence of and skip the first element of the program arguments
    match args.next() {
        Some(_) => {}
        None => {
            eprintln!("Error: {:#?}", "something went wrong");
            std::process::exit(1);
        }
    }

    // matching the `path` element from path arguments
    match args.next() {
        // ----- `path` element exists
        Some(user_input) => {
            match fs::canonicalize(&PathBuf::from(&user_input)) {
                Ok(filepath) => match fs::read_to_string(&filepath) {
                    Ok(contents) => match json.parse(contents) {
                        Ok(safe_value) => {
                            println!("{:#?}", safe_value);
                        }
                        Err(e) => {
                            eprintln!("failed to parse json, Error: {:#?}", e.to_string());
                        }
                    },
                    Err(e) => {
                        eprintln!("failed to read file, Error: {:#?}", e.to_string());
                    }
                },
                Err(e) => {
                    eprintln!("filename: {:#?}, Error: {:#?}", user_input, e.to_string());
                }
            };
        }

        // ----- `path` element doesn't exists
        None => {
            eprintln!("Error: {:#?}", "no input argument provided");
        }
    }
}
