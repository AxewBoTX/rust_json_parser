use serde_json::Value;
use std::{env, fs, path::PathBuf};

fn main() {
    let mut args = env::args();

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
                    Ok(contents) => {
                        let parsed: Value = serde_json::from_str(&contents).unwrap();
                        println!("{:#?}", parsed);
                    }
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
