use std::process::*;
use std::env::*;
use std::fs::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tokens {
    Add,
    Sub,
    Push,
    Pop,
    Int(i64),
}

#[derive(Debug)]
enum Errors {
    PeekLeftOfb,
    PeekRight,
    OutOfBounds
}

#[derive(Debug)]
struct Durian {
    token: i64,
    line: i64,
}

fn tokenize(src: String, dur: &mut Durian) -> Vec<Tokens> {
    let chars: Vec<char> = src.chars().collect();
    let mut tokens: Vec<Tokens> = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        match c {
            '+' => { tokens.push(Tokens::Add); i += 1; dur.token += 1; }
            '-' => { tokens.push(Tokens::Sub); i += 1; dur.token += 1; }
            c if c.is_alphabetic() => {
                let mut string: String = String::new();
                
                while i < chars.len() && chars[i].is_alphabetic(){
                    string.push(chars[i]);
                    i += 1;
                }
                
                match string.as_str() {
                    "push" => { tokens.push(Tokens::Push); dur.token += 1; }
                    "pop" => { tokens.push(Tokens::Pop); dur.token += 1; }
                    _ => { eprintln!("Unrecognized string: {} at token: {:?} at line: {:?}", string, dur.token, dur.line); exit(1); }
                }
            }
            c if c.is_digit(10) => {
                let mut string: String = String::new();

                while i < chars.len() && chars[i].is_digit(10) {
                    string.push_str(&chars[i].to_string());
                    i += 1;
                }

                let num = string.parse::<i64>();

                match num {
                    Ok(n) => { tokens.push(Tokens::Int(n)); dur.token += 1; }
                    Err(e) => { eprintln!("Could not parse int: {}, error: {}", string, e); exit(1); }
                }
            }
            c if c.is_whitespace() => {
                if c == '\n' {
                    dur.line += 1;
                }
                i += 1;
            }
            _ => {
                eprintln!("Unrecognized char: {} at token {:?} at line {:?}", c, dur.token, dur.line);
                exit(1);
            }
        }
    }
    tokens
}

fn peek_l(tokens: &Vec<Tokens>, index: &usize) -> Result<Tokens, Errors> {
    if *index == 0 {
        return Err(Errors::PeekLeftOfb);
    }

    let i = (*index - 1) as usize;

    Ok(tokens[i])
}

fn peek_r(tokens: &Vec<Tokens>, index: &usize) -> Result<Tokens, Errors> {
    if *index == 0 {
        return Err(Errors::PeekLeftOfb);
    }

    let i = (*index + 1) as usize;

    Ok(tokens[i])
}

fn build_program(tokens: &Vec<Tokens>, dur: &mut Durian) {
    let mut string_builder: String = String::new();
    let mut index: usize = 0;
    let mut tok = 0;
    let mut line = 1;

    while index < tokens.len() {
        let c = tokens[index];

        match c {
            Tokens::Add => {
                let l = match peek_l(&tokens, &index) {
                    Ok(t) => t,
                    Err(_) => { 
                        eprintln!("Tried to peek the token previous to the plus op at token {} at line {} but went out of bounds", tok, line); 
                        exit(1); 
                    }
                };
                
                if matches!(l, Tokens::Int(_)) || matches!(l, Tokens::Pop) {
                    eprintln!("Error got token: {:?}, but expected an int or a pop op.\nError occurred at token {} at line {}", l, tok, line);
                    exit(1);
                }
                
            }
        }
        index += 1;
    }
}

fn main() {
    let mut dur = Durian {
        token: 1,
        line: 1,
    };
    
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        eprintln!("You did not pass enough args");
        exit(1);
    }

    let file_name = args[1].clone();

    let file = read_to_string(file_name);

    let src = match file {
        Ok(src) => { src }
        Err(e) => { eprintln!("Could not read file, error {}", e); exit(1) }
    };

    let toks = tokenize(src, &mut dur);
    println!("{:?}", toks);
}
