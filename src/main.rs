use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    println!("{}", std::env::current_dir().unwrap().display());

    const TAB: &str = "  ";
    let separator = "*".repeat(100);

    println!("fileName:");
    let mut file_name = String::new();
    std::io::stdin().read_line(&mut file_name).unwrap();
    let file_name = file_name.trim();

    println!("regex:");
    let mut regex_str = String::new();
    std::io::stdin().read_line(&mut regex_str).unwrap();
    let regex = Regex::new(regex_str.trim()).unwrap();

    let mut lines_found = Vec::new();

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let mut line_number = 0;

    for line in reader.lines() {
        line_number += 1;
        let line = line.unwrap();

        if regex.is_match(&line) {
            lines_found.push(Line { line_number, line });
        }
    }

    println!("{}", separator);

    if lines_found.is_empty() {
        println!("\nNo lines found with regex.");
    } else {
        println!("\nLines found:");

        let pad_size = lines_found.last().unwrap().line_number.to_string().len() + 1;

        for line in lines_found {
            let print = format!("{}=> {: >pad_size$} | {}", TAB, line.line_number, line.line, pad_size = pad_size);
            println!("{}", print);
        }
    }

    println!("\n{}\n", separator);
}

struct Line {
    line_number: i32,
    line: String,
}