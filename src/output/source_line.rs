use lexer::Lexer;
use colored::*;
use std::fs::OpenOptions;
use std::io::Write;

pub fn print_line_char_at_invalid_state(lexer: &Lexer) {
    let line_index = lexer.source_buffer.char_to_line(lexer.current_index);
    let char_index_in_line = lexer.current_index - lexer.source_buffer.line_to_char(line_index);
    let line = lexer.source_buffer.line(line_index);
    let mut line = line.to_string();
    line.pop();

    let output_line1 = format!("On line {}:", line_index + 1);
    let output_line2 = format!("{}", &line[..char_index_in_line]);
    let output_line3 = format!("{}", &line[char_index_in_line..char_index_in_line+1]);
    let output_line4 = format!("{}", &line[char_index_in_line+1..]);

    let mut file = OpenOptions::new().write(true).append(true).open("error.log").unwrap();
    println!("{}", output_line1.bright_white().on_purple().bold());
    writeln!(file, "{}", String::from(output_line1)).unwrap();
    print!("{}", output_line2.bright_white().on_purple().bold());
    write!(file, "{}", String::from(output_line2)).unwrap();
    print!("{}", output_line3.bright_red().on_white().bold());
    write!(file, "{}", String::from(output_line3)).unwrap();
    println!("{}", output_line4.bright_white().on_purple().bold());
    writeln!(file, "{}", String::from(output_line4)).unwrap();
    writeln!(file, "---------------------------------------------").unwrap();
}
