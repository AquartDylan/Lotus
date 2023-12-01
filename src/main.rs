use std::fs;

mod lexer;

fn main() {
    let input = fs::read_to_string("test.lts").expect("Unable to read file");
    lexer::tokenize(&input);
    println!("Hello World!");
}
