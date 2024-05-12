pub mod decoder;
pub mod sip_parser;
pub mod text_reader;

use std::fs;
use std::io::BufReader;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let text = fs::read_to_string(file_path).unwrap();
    let reader = text_reader::TextReader::new(BufReader::new(text.as_bytes()));
    let mut parser = sip_parser::SipParser::new(reader);
    let request = parser.parse_request();

    print!("{}", request.to_string());
}