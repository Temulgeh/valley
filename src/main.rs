#[macro_use]
extern crate lalrpop_util;

mod types;
mod ast;
mod check;
mod interpreter;
mod repl;
lalrpop_mod!(parser);


fn main() {
    display_title("valley - LALRPOP test");
    repl::start_repl_loop();
}

fn display_title(title: &str) {
    let bar = "═".repeat(title.len() + 2);
    println!("\n╒{bar}╕\n│ {title} │\n╘{bar}╛\n");
}
