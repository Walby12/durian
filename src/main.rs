use std::process::*;
use std::env::*;
use std::fs::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tokens {
    Add,
    Sub,
    Push,
    Pop,
    Int(i64),
    Print,
    Newline,
}

#[derive(Debug)]
enum Errors {
    PeekLeftOfb,
    PeekRightOfb,
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
            c if c.is_alphabetic() => {
                let mut string: String = String::new();
                
                while i < chars.len() && chars[i].is_alphabetic(){
                    string.push(chars[i]);
                    i += 1;
                }
                
                match string.as_str() {
                    "push" => { tokens.push(Tokens::Push); dur.token += 1; }
                    "pop" => { tokens.push(Tokens::Pop); dur.token += 1; }
                    "add" => { tokens.push(Tokens::Add); dur.token += 1; }
                    "sub" => { tokens.push(Tokens::Sub); dur.token += 1; }
                    "print" => { tokens.push(Tokens::Print); dur.token += 1; }
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
                    Ok(n) => { tokens.push(Tokens::Int(n)); }
                    Err(e) => { eprintln!("Could not parse int: {}, error: {}", string, e); exit(1); }
                }
            }
            c if c.is_whitespace() => {
                if c == '\n' {
                    dur.line += 1;
                    dur.token = 0;
                    tokens.push(Tokens::Newline);
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
    if *index == tokens.len() {
        return Err(Errors::PeekRightOfb);
    }

    let i = (*index + 1) as usize;

    Ok(tokens[i])
}

fn build_program(tokens: &Vec<Tokens>, file_path: String) {
    let mut string_builder: String = String::new();
    let mut index: usize = 0;
    let mut tok = 0;
    let mut line = 1;
    let mut msg = String::new();
    let mut stack_all = 0;

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();

    msg.push_str("format ELF64\n");
    msg.push_str("section \".data\" writable\n\tfmt db \"%c\", 10, 0\n");
    msg.push_str("section \".text\" executable\n");
    msg.push_str("public main\nextrn printf\nmain:\n");

    while index < tokens.len() {
        let c = tokens[index];

        match c {
            Tokens::Add => {
                tok += 1;

                msg.push_str("\tpop rax\n");
                msg.push_str("\tpop rbx\n");
                msg.push_str("\tadd rax, rbx\n");
                msg.push_str("\tpush rax\n");
                stack_all -= 8;
            }
            Tokens::Push => {
                tok += 1;

                let r = match peek_r(&tokens, &index) {
                    Ok(t) => t,
                    Err(_) => {
                        eprintln!(
                            "Tried to peek the token next to the push op at token {} at line {} but went out of bounds",
                            tok + 1, line
                        );
                        exit(1);
                    }
                };

                if !matches!(r, Tokens::Int(..)) {
                    eprintln!(
                        "Error got token: {:?}, but expected an Int.\nError occurred at token {} at line {}",
                        r,
                        tok + 1,
                        line
                    );
                    exit(1);
                }

                if let Tokens::Int(n) = r {
                    msg.push_str(&format!("\tpush {}\n", n));
                }

                index += 1;
                stack_all += 8;
            }
            Tokens::Pop => {
                tok += 1;
                msg.push_str("\tpop rbx\n");
                stack_all -= 8;
            }
            Tokens::Newline => {
                line += 1;
                tok = 0;
            }
            Tokens::Print => {
                stack_all -= 8;
                if stack_all % 16 != 0 {
                    eprintln!("The stack may be misalligned and the program may go into a seg fault for this print call.\nWarinig emitted at token: {} at line: {}", tok + 1, line);
                }
                msg.push_str(&format!("\tmov rdi, fmt\n\tpop rax\n\tadd rax, '0'\n\tmov rsi, rax\n\txor eax, eax\n\tcall printf\n"))
            }
            _ => {
                eprintln!("Unexpected token {:?} at token: {} at line {}", c, tok + 1, line);
                exit(1);
            }
        }
        index += 1;
    }

    msg.push_str("\txor eax, eax\n\tret\n");
    writeln!(file, "{}", msg).unwrap();
}

fn strip_end(filename: String) -> String {
    let path = Path::new(&filename);
    match path.file_stem() {
        Some(stem) => stem.to_string_lossy().into_owned(),
        None => filename,
    }
}

fn exec_program(file_path: String) {
    let mut arg2 = strip_end(file_path.clone());
    arg2.push_str(".o");
    let arg3 = strip_end(file_path.clone());

    let comm_1 = Command::new("fasm")
        .arg(file_path)
        .arg(arg2.clone())
        .status()
        .expect("Could not run fasm command");
    
    let comm_2 = Command::new("gcc")
        .arg("-no-pie")
        .arg(arg2)
        .arg("-o")
        .arg(arg3)
        .status()
        .expect("Could not run gcc command");
}

fn main() {
    let mut dur = Durian {
        token: 1,
        line: 1,
    };
    
    let args: Vec<String> = args().collect();
    let mut out_file_name = String::new();

    if args.len() < 2 {
        eprintln!("You did not pass enough args");
        exit(1);
    } else if args.len() == 2 {
        out_file_name = strip_end(args[1].clone());
        out_file_name.push_str(".s");
    } else if args.len() == 3 {
        out_file_name = strip_end(args[2].clone());
        if out_file_name != args[2].clone() {
            eprintln!("Invalid output name: {}", args[2].clone());
            exit(1);
        }
        out_file_name.push_str(".s");
    }

    let file_name = args[1].clone();

    let file = read_to_string(file_name);

    let src = match file {
        Ok(src) => { src }
        Err(e) => { eprintln!("Could not read file, error {}", e); exit(1) }
    };

    let toks = tokenize(src, &mut dur);
    build_program(&toks, out_file_name.clone());
    exec_program(out_file_name);
}
