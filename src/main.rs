use std::fs::File;
use std::io::Write;

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

fn parse_args(file_str: &Option<String>) -> &String {
    match file_str {
        Some(file_str) => file_str,
        None => {
            eprintln!("No file provided");
            std::process::exit(1);
        }
    }
}

fn next_power_of_2(n: usize) -> usize {
    let mut size = 2usize;
    while size < n {
        size *= 2;
    }
    size
}

fn main() {
    let padding = 0xFF;

    let filename = std::env::args().nth(1);
    let mut file = parse_filename(parse_args(&filename));
    let current_size = file.metadata().unwrap().len() as usize;

    println!("File: {}", filename.unwrap());
    println!("Current size: {}", current_size);

    // Find next power of 2
    let new_size = next_power_of_2(current_size);
    println!("New size: {}", new_size);

    // if new size is bigger than current size, fill with padding until new size
    if new_size > current_size {
        let diff = new_size - current_size;
        let buffer = vec![padding; diff];
        file.write_all(&buffer).unwrap();
    }
}
