use std::fs::File;
use std::io::Write;
use clap::Parser;
use unbytify::unbytify;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
    #[arg(short, long)]
    size: String,
    #[arg(short, long)]
    padding: String,
}

fn parse_filename(filename: &str) -> File {
    let result = File::options().read(true).write(true).append(true).open(&filename);
    match result {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Cannot open file: {}", filename);
            std::process::exit(1);
        }
    }
}

fn parse_size(size: &str) -> usize {
    let result = unbytify(size);
    match result {
        Ok(size) => size as usize,
        Err(_) => {
            eprintln!("Invalid size: {}", size);
            std::process::exit(1);
        }
    }
}

fn parse_padding(padding: &str) -> u8 {
    let result = padding.parse::<u8>();
    match result {
        Ok(padding) => padding,
        Err(_) => {
            eprintln!("Invalid padding: {}", padding);
            std::process::exit(1);
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut file = parse_filename(&args.filename);
    let new_size = parse_size(&args.size);
    let padding = parse_padding(&args.padding);
    let current_size = file.metadata().unwrap().len() as usize;

    println!("File: {}", args.filename);
    println!("Current size: {}", current_size);
    println!("New size: {}", new_size);

    // if new size is bigger than current size, fill with padding until new size
    if new_size > current_size {
        let diff = new_size - current_size;
        let buffer = vec![padding; diff];
        file.write_all(&buffer).unwrap();
    }
}
