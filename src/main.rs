use std::{env, fs, path::PathBuf};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    match fs::canonicalize(&PathBuf::from(&args[1])) {
        Ok(filepath) => match fs::read_to_string(&filepath) {
            Ok(contents) => {
                println!("{:#?}", contents);
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
