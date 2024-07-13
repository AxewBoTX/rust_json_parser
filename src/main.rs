use std::{env, fs, path::PathBuf};

use json_parser;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let json = json_parser::JsonParser::new();
    match fs::canonicalize(&PathBuf::from(&args[1])) {
        Ok(filepath) => match fs::read_to_string(&filepath) {
            Ok(contents) => {
                let tokens = json.tokenize(contents);
                println!("Parsed JSON: {:#?}", json.parse(tokens));
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

#[cfg(test)]
mod test {
    #[test]
    pub fn test_stuff() {
        let mut list = vec![1, 2, 3, 4, 5, 6, 7].into_iter().peekable();
        while let Some(element) = list.next() {
            if element > 1 && element < 4 {
                while let Some(thing) = list.next() {
                    if thing == 5 {
                        println!("this is five inside");
                        break;
                    }
                }
            }
            if element == 3 {
                println!("this is three")
            }
            if element == 4 {
                println!("this is four")
            }
            if element == 5 {
                println!("this is five outside")
            }
            if element == 6 {
                println!("this is six")
            }
        }
    }
}
