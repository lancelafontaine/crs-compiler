use colored::*;
use std::collections::HashMap;


lazy_static! {
    static ref ERROR_CODES: HashMap<i32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1, "The input file path provided does not exist on the filesystem.");
        m
    };
    static ref UNDEFINED_CODE: i16 = 999;
}

pub fn error(code: i32) -> i32 {
    return match ERROR_CODES.get(&code) {
        Some(message) => {
            println!("{}", format!("ERROR CODE {}", code).on_bright_red().bright_yellow().bold());
            println!("{}", message.bright_red());
            code
        }
        None => {
            println!("{}", format!("ERROR CODE {}", *UNDEFINED_CODE).on_bright_red().bright_yellow().bold());
            println!("{}", "Unspecified error.".bright_red());
            999
        }
    }
}
