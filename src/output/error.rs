use colored::*;
use std::collections::HashMap;


pub enum ErrorType {
    Normal(i32),
    RecoverableError(i32, &'static str),
    UnrecoverableError(i32, &'static str)
}

lazy_static! {
    static ref ERROR_CODES: HashMap<i32, ErrorType> = {
        let mut m = HashMap::new();
        m.insert(1, ErrorType::UnrecoverableError(1, "The input file path provided does not exist on the filesystem."));
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
                    println!("{}", format!("WARNING CODE {}", code).on_bright_yellow().bold());
                    println!("{}", message.bright_yellow());
                    ErrorType::RecoverableError(code, message)
                },
                ErrorType::UnrecoverableError(code, message)=> {
                    println!("{}", format!("ERROR CODE {}", code).on_bright_red().bright_yellow().bold());
                    println!("{}", message.bright_red());
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
