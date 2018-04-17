use colored::*;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use output::write_to_error_log;

pub enum ErrorType {
    Normal(i32),
    RecoverableError(i32, &'static str),
    UnrecoverableError(i32, &'static str),
}

lazy_static! {
    static ref ERROR_CODES: HashMap<i32, ErrorType> = {
        let mut m = HashMap::new();
        m.insert(1, ErrorType::UnrecoverableError(1, "GENERAL: The input file path provided does not exist on the filesystem."),
        );
        m.insert(2, ErrorType::RecoverableError(2, "LEXER: Input character is unrecognized."));
        m.insert(3, ErrorType::RecoverableError(3, "LEXER: Invalid syntax: error state reached. Resetting FSM to initial state."));
        m.insert(4, ErrorType::RecoverableError(4, "CODEGEN: Unable to write generated assembly code to file."));
        m.insert(5, ErrorType::RecoverableError(5, "PARSER: Pop error."));
        m.insert(6, ErrorType::RecoverableError(6, "PARSER: Scan error."));
        m.insert(7, ErrorType::RecoverableError(7, "PARSER: Reached end of input without finishing parsing."));
        m.insert(8, ErrorType::RecoverableError(8, "PARSER: Input token cannot be matched to a valid parse tree."));
        m.insert(9, ErrorType::UnrecoverableError(9, "TYPECHECK: AST root node is not ProgramFamily."));
        m.insert(10, ErrorType::UnrecoverableError(10, "TYPECHECK: There are no nodes in the generated AST."));
        m.insert(11, ErrorType::UnrecoverableError(11, "TYPECHECK: Unexpected record type for a class declaration table (not a function or a variable)."));
        m.insert(12, ErrorType::UnrecoverableError(12, "TYPECHECK: A data member of a class has been declared twice."));
        m.insert(13, ErrorType::UnrecoverableError(13, "TYPECHECK: A class method was defined without being declared, or vice-versa."));
        m.insert(14, ErrorType::UnrecoverableError(14, "TYPECHECK: A type, identifier or array size without a declaration or expression context is unusable."));
        m.insert(15, ErrorType::UnrecoverableError(15, "TYPECHECK: A class method is defined for a class that does not exist."));
        m.insert(16, ErrorType::RecoverableError(16, "TYPECHECK: Attempting to set a field in the table for the root table, which does not have a parent record."));
        m.insert(17, ErrorType::UnrecoverableError(17, "TYPECHECK: That symbol table record or table does not exist."));
        m.insert(18, ErrorType::UnrecoverableError(18, "TYPECHECK: Operation only valid for record nodes performed on non-record node."));
        m.insert(19, ErrorType::UnrecoverableError(19, "TYPECHECK: Operation only valid for table nodes performed on non-table node."));
        m.insert(20, ErrorType::UnrecoverableError(20, "TYPECHECK: Attempting to instantiate a class that was not declared."));
        m.insert(21, ErrorType::UnrecoverableError(21, "TYPECHECK: A variable, function or class was declared twice."));
        m.insert(22, ErrorType::UnrecoverableError(22, "TYPECHECK: There is a cyclic dependency between your class inheritance lists or object members."));
        m
    };
}

pub fn error(code: i32) -> ErrorType {
    match ERROR_CODES.get(&code) {
        Some(error_type) => match *error_type {
            ErrorType::Normal(code) => ErrorType::Normal(code),
            ErrorType::RecoverableError(code, message) => {
                let line1 = format!("WARNING CODE {}", code);
                let line2 = message;

                println!("{}", line1.on_bright_yellow().bright_black().bold());
                write_to_error_log(String::from(line1));
                println!("{}", line2.bright_yellow());
                write_to_error_log(String::from(line2));
                write_to_error_log(String::from("---------------------------------------------"));
                ErrorType::RecoverableError(code, message)
            }
            ErrorType::UnrecoverableError(code, message) => {
                let line1 = format!("ERROR CODE {}", code);
                let line2 = message;

                println!("{}", line1.on_bright_red().bright_yellow().bold());
                write_to_error_log(String::from(line1));
                println!("{}", line2.bright_red());
                write_to_error_log(String::from(line2));
                write_to_error_log(String::from("---------------------------------------------"));
                ErrorType::UnrecoverableError(code, message)
            }
        },
        None => {
            const UNDEFINED_CODE: i32 = 999;
            println!(
                "{}",
                format!("ERROR CODE {}", UNDEFINED_CODE)
                    .on_bright_red()
                    .bright_yellow()
                    .bold()
            );
            let message = "Unspecified error.";
            println!("{}", message.bright_red());
            ErrorType::UnrecoverableError(UNDEFINED_CODE, message)
        }
    }
}
