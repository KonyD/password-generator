#![allow(unused)]

use std::char::from_u32;

use clap::Parser;
use rand::Rng;

/// A basic CLI for generating random passwords
#[derive(Parser)]
struct Cli {
    /// Length of the password
    length: i32,
}

fn generate_password(length: i32) -> String {
    let password_length = length;
    let mut result = String::new();

    for _ in 0..password_length {
        let number = rand::thread_rng().gen_range(48..122);
        let char = from_u32(number).unwrap();
        result.push(char);
    }

    result
}

fn main() {
    let args = Cli::parse();
    let password = generate_password(args.length);
    
    println!("{}", password);
}
