use std::io::{self, Write};
use lalrpop_util::ParseError;
use crate::parser::StatementParser;
use crate::interpreter::Interpreter;


pub fn start_repl_loop() {
    // TODO: somehow stop execution if user is piping text
    let parser = StatementParser::new();
    let mut interpreter = Interpreter::new();
    let mut total_input = String::new();
    while let Some(input) = query_line(
        if total_input.len() == 0 { "> " } else { "  " }
    ) {
        total_input += &input;
        match parser.parse(&total_input) {
            Ok(ast) => println!("{}", interpreter.eval(&ast)),
            Err(ParseError::UnrecognizedEof { .. }) => continue,
            Err(error) => println!("Error: {}", error)
        }
        total_input.clear();
    }
}

fn query_line(prompt: &str) -> Option<String> {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok().and(Some(line))
}
