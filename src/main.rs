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
    Div,
    IDiv,
    IMul,
    Mul,
    Mod,
    Dup,
    Push,
    Pop,
    Swap,
    Int(i64),
    PrintInt,
    PutChar,
    Newline,
    Label,
    Ident(usize),
    Jmp,
    JmpE, 
}

struct Interner {
    strings: Vec<String>,
}

impl Interner {
    fn new() -> Self {
        Interner { strings: Vec::new() }
    }
    fn intern(&mut self, s: String) -> usize {
        let id = self.strings.len();
        self.strings.push(s);
        id
    }
    fn resolve(&self, id: usize) -> &str {
        &self.strings[id]
    }
}

#[derive(Debug)]
enum Errors {
    PeekLeftOfb,
    PeekRightOfb,
    OutOfBounds
}

fn tokenize(src: String, intern: &mut Interner) -> Vec<Tokens> {
    let chars: Vec<char> = src.chars().collect();
    let mut tokens: Vec<Tokens> = Vec::new();
    let mut i = 0;
    let mut token = 2;
    let mut line = 1;

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
                    "push" => { tokens.push(Tokens::Push); token += 1; }
                    "pop" => { tokens.push(Tokens::Pop); token += 1; }
                    "add" => { tokens.push(Tokens::Add); token += 1; }
                    "sub" => { tokens.push(Tokens::Sub); token += 1; }
                    "div" => { tokens.push(Tokens::Div); token += 1; }
                    "idiv" => { tokens.push(Tokens::IDiv); token += 1; }
                    "imul" => { tokens.push(Tokens::IMul); token += 1; }
                    "mul" => { tokens.push(Tokens::Mul); token += 1; }
                    "swap" => { tokens.push(Tokens::Swap); token += 1; }
                    "mod" => { tokens.push(Tokens::Mod); token += 1; }
                    "dup" => { tokens.push(Tokens::Dup); token += 1; }
                    "printint" => { tokens.push(Tokens::PrintInt); token += 1; }
                    "putchar" => { tokens.push(Tokens::PutChar); token += 1; }
                    "label" => { tokens.push(Tokens::Label); token += 1; }
                    "jmp" => { tokens.push(Tokens::Jmp); token += 1; }
                    "je" => { tokens.push(Tokens::JmpE); token += 1; }
                    _ => { 
                        let id = intern.intern(string);
                        tokens.push(Tokens::Ident(id));
                        token += 1;
                    }
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
                    line += 1;
                    token = 0;
                    tokens.push(Tokens::Newline);
                }
                i += 1;
            }
            _ => {
                eprintln!("Unrecognized char: {} at token {} at line {}", c, token + 1, line);
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

fn build_program(tokens: &Vec<Tokens>, file_path: String, intern: &Interner) {
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
    msg.push_str("section \".data\" writable\n\tfmt db \"%d\", 10, 0\n");
    msg.push_str("section \".text\" executable\n");
    msg.push_str("public main\nextrn printf\nextrn putchar\nmain:\n");

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
            Tokens::Sub => {
                tok += 1;

                msg.push_str("\tpop rax\n");
                msg.push_str("\tpop rbx\n");
                msg.push_str("\tsub rax, rbx\n");
                msg.push_str("\tpush rax\n");
                stack_all -= 8;
            }
            Tokens::Div => {
                tok += 1;
                
                msg.push_str("\tpop rax\n");
                msg.push_str("\tpop rbx\n");
                msg.push_str("\txor rdx, rdx\n");
                msg.push_str("\tdiv rbx\n");
                msg.push_str("\tpush rax\n");
                stack_all -= 8;
            }
            Tokens::IDiv => {
                tok += 1;

                msg.push_str("\tpop rax\n");
                msg.push_str("\tpop rbx\n");
                msg.push_str("\tcqo\n");
                msg.push_str("\tidiv rbx\n");
                msg.push_str("\tpush rax\n");
                stack_all -= 8;
            }
            Tokens::Mul => {
                tok += 1;

                msg.push_str("\tpop rax\n");
                msg.push_str("\tpop rbx\n");
                msg.push_str("\txor rdx, rdx\n");
                msg.push_str("\tmul rbx\n");
                msg.push_str("\tpush rax\n");
                stack_all -= 8;
            }
            Tokens::IMul => {
                tok += 1;

                msg.push_str("\tpop rax\n");
                msg.push_str("\tpop rbx\n");
                msg.push_str("\tcqo\n");
                msg.push_str("\timul rbx\n");
                msg.push_str("\tpush rax\n");
                stack_all -= 8;
            }
            Tokens::Mod => {
                tok += 1;
                
                msg.push_str("\tpop rax\n");
                msg.push_str("\tpop rbx\n");
                msg.push_str("\tcqo\n");
                msg.push_str("\tidiv rbx\n");
                msg.push_str("\tpush rdx\n");
                stack_all -= 8;
            }
            Tokens::Swap => {
                tok += 1;

                msg.push_str("\tpop rax\n");
                msg.push_str("\tpop rbx\n");
                msg.push_str("\tpush rax\n");
                msg.push_str("\tpush rbx\n");
            }
            Tokens::Dup => {
                tok += 1;

                msg.push_str("\tmov rax, [rsp]\n");
                msg.push_str("\tpush rax\n");
                stack_all += 8;
            }
            Tokens::Push => {
                tok += 2;

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
            Tokens::PrintInt => {
                tok += 1;
                stack_all -= 8;
                if stack_all % 16 != 0 {
                    eprintln!("The stack is disalligned and the program will go in seg fault on this printint op.\nError occurred at token: {} at line: {}", tok + 1, line);
                }

                msg.push_str("\tmov rdi, fmt\n");
                msg.push_str("\tpop rsi\n");
                msg.push_str("\txor eax, eax\n");

                msg.push_str("\tsub rsp, 8\n");
                msg.push_str("\tcall printf\n");
                msg.push_str("\tadd rsp, 8\n");
            }
            Tokens::PutChar => {
                tok += 1;
                stack_all -= 8;
                if stack_all % 16 != 0 {
                    eprintln!("The stack is disalligned and the program will go in seg fault on this putchar op.\nError occurred at token: {} at line: {}", tok + 1, line);
                }

                msg.push_str("\tpop rdi\n");
                msg.push_str("\tsub rsp, 8\n");
                msg.push_str("\tcall putchar\n");
                msg.push_str("\tadd rsp, 8\n");
            }
            Tokens::Label => {
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

                if !matches!(r, Tokens::Ident(..)) {
                    eprintln!(
                        "Error got token: {:?}, but expected an Identifier.\nError occurred at token {} at line {}",
                        r,
                        tok + 1,
                        line
                    );
                    exit(1);
                }

                if let Tokens::Ident(n) = r {
                    msg.push_str(&format!("{}:\n", intern.resolve(n)));
                }
                index += 1;
            }
            Tokens::Jmp => {
                tok += 2;

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

                if !matches!(r, Tokens::Ident(..)) {
                    eprintln!(
                        "Error got token: {:?}, but expected an Identifier.\nError occurred at token {} at line {}",
                        r,
                        tok + 1,
                        line
                    );
                    exit(1);
                }

                if let Tokens::Ident(n) = r {
                    msg.push_str(&format!("jmp {}\n", intern.resolve(n)));
                }
                index += 1;
            }
            Tokens::JmpE => {
                tok += 2;
                
                let r1 = match peek_r(&tokens, &index) {
                    Ok(t) => t,
                    Err(_) => {
                        eprintln!(
                            "Tried to peek the token next to the push op at token {} at line {} but went out of bounds",
                            tok + 1, line
                        );
                        exit(1);
                    }
                };

                if !matches!(r1, Tokens::Int(..)) {
                    eprintln!(
                        "Error got token: {:?}, but expected an Int.\nError occurred at token {} at line {}",
                        r1,
                        tok + 1,
                        line
                    );
                    exit(1);
                }

                let r2 = match peek_r(&tokens, &(index + 1)) {
                    Ok(t) => t,
                    Err(_) => {
                        eprintln!(
                            "Tried to peek the token next to the push op at token {} at line {} but went out of bounds",
                            tok + 1, line
                        );
                        exit(1);
                    }
                };

                if !matches!(r2, Tokens::Ident(..)) {
                    eprintln!(
                        "Error got token: {:?}, but expected an Identifier.\nError occurred at token {} at line {}",
                        r2,
                        tok + 1,
                        line
                    );
                    exit(1);
                }

                if let Tokens::Int(n) = r1 {
                    msg.push_str(&format!("\tpop rax\n\tcmp rax, {}\n", n));
                }
                if let Tokens::Ident(n) = r2 {
                    msg.push_str(&format!("\tje {}\n", intern.resolve(n)));
                }
                index += 2;
            }
            _ => {
                if let Tokens::Ident(n) = c {
                    eprintln!("Unexpected token {:?} at token: {} at line {}", intern.resolve(n), tok + 1, line);
                } else {
                    eprintln!("Unexpected token {:?} at token: {} at line {}", c, tok + 1, line);
                }
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
    let mut inter = Interner::new();
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

    let toks = tokenize(src, &mut inter);
    build_program(&toks, out_file_name.clone(), &inter);
    exec_program(out_file_name);
}
