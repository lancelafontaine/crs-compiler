use colored::*;
use lexer::Lexer;
use std::fs::OpenOptions;
use std::io::Write;
use output::write_to_error_log;

pub fn print_line_char_at_invalid_state(lexer: &Lexer) {
    let line_index = lexer.source_buffer.char_to_line(lexer.current_index);
    let char_index_in_line = lexer.current_index - lexer.source_buffer.line_to_char(line_index);
    let line = lexer.source_buffer.line(line_index);
    let mut line = line.to_string();
    line.pop();

    let output_line1 = format!("On line {}:", line_index + 1);
    let output_line2 = format!("{}", &line[..char_index_in_line]);
    let output_line3 = format!("{}", &line[char_index_in_line..]);

    println!("{}", output_line1.bright_white().on_purple().bold());
    write_to_error_log(String::from(output_line1));
    print!("{}", output_line2.bright_white().on_purple().bold());
    write_to_error_log(String::from(output_line2));
    println!("{}", output_line3.bright_white().on_purple().bold());
    write_to_error_log(String::from(output_line3));
    write_to_error_log(String::from("----------------------------------------------"));
}
