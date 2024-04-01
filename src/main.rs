use std::io::{BufRead, BufReader};
use colored::*;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() {

    let args = Cli::parse();
    let path_str = args.path.to_str().unwrap().cyan();
    println!("pattern: {} path: {} \n", args.pattern.cyan() , path_str);
    let search_file= std::fs::File::open(&args.path).expect("Could not read file");
    let mut reader = BufReader::new(search_file);
    
    let mut line = String::new();
    let mut match_count: u32 = 0;
    while (reader.read_line(&mut line)).unwrap() != 0 {
        if line.contains(&args.pattern) {
            match_count += 1;
            println!("{}",line);
            
        }
        line.clear();
    }
    if match_count != 0 {
        println!("match count: {}", match_count.to_string().cyan()); 
    } else {
       println!("{} could was not found in {}.", args.pattern.blue(), path_str.blue()); 
    }
}

