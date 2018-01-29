use colored::*;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

pub enum ErrorType {
    Normal(i32),
    RecoverableError(i32, &'static str),
    UnrecoverableError(i32, &'static str)
}

lazy_static! {
    static ref ERROR_CODES: HashMap<i32, ErrorType> = {
        let mut m = HashMap::new();
        m.insert(1, ErrorType::UnrecoverableError(1, "The input file path provided does not exist on the filesystem."));
        m.insert(2, ErrorType::RecoverableError(2, "Input character is unrecognized."));
        m.insert(3, ErrorType::RecoverableError(3, "Invalid syntax: error state reached. Resetting FSM to initial state."));
        m
    };
}

pub fn error(code: i32) -> ErrorType {
    match ERROR_CODES.get(&code) {
        Some(error_type) => {
            match *error_type {
                ErrorType::Normal(code) => {
                    ErrorType::Normal(code)
                },
                ErrorType::RecoverableError(code, message)=> {
                    let line1 = format!("WARNING CODE {}", code);
                    let line2 = message;

                    let mut file = OpenOptions::new().write(true).append(true).open("error.log").unwrap();
                    println!("{}", line1.on_bright_yellow().bright_black().bold());
                    writeln!(file, "{}", String::from(line1)).unwrap();
                    println!("{}", line2.bright_yellow());
                    writeln!(file, "{}", String::from(line2)).unwrap();
                    writeln!(file, "---------------------------------------------").unwrap();
                    ErrorType::RecoverableError(code, message)
                },
                ErrorType::UnrecoverableError(code, message)=> {
                    let line1 = format!("ERROR CODE {}", code);
                    let line2 = message;

                    let mut file = OpenOptions::new().write(true).append(true).open("error.log").unwrap();
                    println!("{}", line1.on_bright_red().bright_yellow().bold());
                    writeln!(file, "{}", String::from(line1)).unwrap();
                    println!("{}", line2.bright_red());
                    writeln!(file, "{}", String::from(line2)).unwrap();
                    writeln!(file, "---------------------------------------------").unwrap();
                    ErrorType::UnrecoverableError(code, message)
                }
            }
        }
        None => {
            const UNDEFINED_CODE: i32 = 999;
            println!("{}", format!("ERROR CODE {}", UNDEFINED_CODE).on_bright_red().bright_yellow().bold());
            let message = "Unspecified error.";
            println!("{}", message.bright_red());
            ErrorType::UnrecoverableError(UNDEFINED_CODE, message)
        }
    }
}
