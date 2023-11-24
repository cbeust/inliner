use std::fs;
use clap::Parser;

fn indent(line: &mut String, size: u8) {
    for _ in 0..size {
        line.push_str(" ");
    }
}

#[derive(Parser)]
#[command(author, version, about)]
/// Convert a binary file into a Rust array.
struct Args {
    #[arg(short, long, default_value_t = 80, help = "Max number of columns")]
    columns: usize,

    #[arg(short, long, default_value_t = 4, help = "Number of spaces for indentation")]
    indent: u8,

    #[arg(short, long, help = "Name of the variable [default: derived from the file name]")]
    variable: Option<String>,

    file_name: String,
}

fn main() {
    let args = Args::parse();
    let bytes = fs::read(args.file_name.clone()).expect("Couldn't open file");
    let variable_name = if let Some(name) = args.variable {
        name
    } else {
        let mut result = String::new();
        for c in args.file_name.clone().chars() {
            result.push(
                match c {
                    'a'..='z' => { c.to_ascii_uppercase() }
                    'A'..='Z' => { c }
                    _ => { '_' }
                }
            )
        }
        result
    };

    println!("const {}: [u8; {}] = [", variable_name, bytes.len());
    let mut current_line = String::new();
    indent(&mut current_line, args.indent);
    for byte in bytes {
        let formatted_byte = format!("0x{:02X}, ", byte);
        if current_line.len() + formatted_byte.len() >= args.columns {
            println!("{}", current_line);
            current_line = String::new();
            indent(&mut current_line, args.indent);
        }
        current_line.push_str(&formatted_byte);
    }
    if ! current_line.is_empty() {
        println!("{}", current_line);
    }
    println!("];");
}
