use md2html::lexer::Lexer;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;
use std::{env, fs};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    let source = args[0].trim();
    println!("source: {}", source);

    let source = PathBuf::from(source);

    if source.extension().unwrap() != "md" {
        eprintln!("Support only markdown");
        exit(1)
    }

    let mut file = fs::File::open(&source).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut lexer = Lexer::new(content);

    let tokens = lexer.scan_tokens();

    tokens.iter().for_each(|t| println!("{:?}", t));

    // TODO: lexer
}
