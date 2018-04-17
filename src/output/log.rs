use std::fs::OpenOptions;
use std::io::Write;
use std::fs;

pub fn initialize_logs() {
    fs::write("logs/0-error.log", "");
    fs::write("logs/1-token.log", "");
    fs::write("logs/2-parse.log", "");
    fs::write("logs/3-ast.log", "");
    fs::write("logs/4-symbol-table.log", "");
}

pub fn write_to_error_log(message: String) {
    let mut file = get_open_file("logs/0-error.log");
    writeln!(file, "{:?}", message);
}

pub fn write_to_token_log(message: String) {
    let mut file = get_open_file("logs/1-token.log");
    writeln!(file, "{:?}", message);
}

pub fn write_to_parse_log(message: String) {
    let mut file = get_open_file("logs/2-parse.log");
    writeln!(file, "{:?}", message);
}

pub fn write_to_ast_log(message: String) {
    let mut file = get_open_file("logs/3-ast.log");
    writeln!(file, "{}", message);
}

pub fn write_to_symbol_table_log(message: String) {
    let mut file = get_open_file("logs/4-symbol-table.log");
    writeln!(file, "{:?}", message);
}

fn get_open_file<'a>(log: &'a str) -> fs::File {
    OpenOptions::new()
        .write(true)
        .append(true)
        .open(log)
        .unwrap()
}


