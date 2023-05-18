use std::io::{self, Write};


mod token;
mod ast;


fn main() {
    // TODO: somehow stop execution if user is piping text
    display_title("valley - tokenizer test");
    while let Some(input) = query_input() {
        println!("{:?}\n", token::TokenIter::new(input.chars()))
    }
}

fn display_title(title: &str) {
    let bar = "═".repeat(title.len() + 2);
    println!("\n╒{bar}╕\n│ {title} │\n╘{bar}╛\n");
}

fn query_input() -> Option<String> {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .ok()
        .and(Some(line))
}
