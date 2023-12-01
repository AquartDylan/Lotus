pub enum Token{
    TokenEof = -1,

    TokenDefinition = -2,
    TokenImport = -3,

    TokenIdentifier = -4,
    TokenNmber = -5,
}

pub fn tokenize(input: &str) {
    let _result: Vec<String> = Vec::new();
    for character in input.chars() {
        if character.is_alphabetic(){
            while(true){
                println!("Joel")
            }
        }
        println!("{character}")
    }
}