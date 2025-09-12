use std::process::*;

#[derive(Debug)]
enum Tokens {
    Add,
    Sub,
    Push,
    Pop,
    Int(i64),
}

fn tokenize(src: String) -> Vec<Tokens> {
    let chars: Vec<char> = src.chars().collect();
    let mut tokens: Vec<Tokens> = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        match c {
            '+' => { tokens.push(Tokens::Add); i += 1; }
            '-' => { tokens.push(Tokens::Sub); i += 1; }
            mut c if c.is_alphabetic() => {
                let mut string: String = String::new();
                
                while i < chars.len() && chars[i].is_alphabetic(){
                    string.push(chars[i]);
                    i += 1;
                }
                
                match string.as_str() {
                    "push" => { tokens.push(Tokens::Push); }
                    "pop" => { tokens.push(Tokens::Pop); }
                    _ => { eprintln!("Unrecognized string: {}", string); exit(1); }
                }
            }
            mut c if c.is_digit(10) => {
                let mut string: String = String::new();

                while i < chars.len() && chars[i].is_digit(10) {
                    string.push_str(&c.to_string());
                    i += 1;
                }

                let num = string.parse::<i64>();

                match num {
                    Ok(n) => { tokens.push(Tokens::Int(n)); }
                    Err(e) => { eprintln!("Could not parse int: {}, error: {}", string, e); exit(1); }
                }
            }
            c if c.is_whitespace() => {
                i += 1;
            }
            _ => {
                eprintln!("Unrecognized char: {}", c);
                exit(1);
            }
        }
    }
    tokens
}

fn main() {
    let toks = tokenize(String::from("+ - push pop 12"));
    println!("{:?}", toks);
}
