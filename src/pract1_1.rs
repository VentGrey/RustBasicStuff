/* Write a program that takes two numbers from the user input
and calculates the addition, substraction and division of both
numbers */

use std::io;

pub fn main() {
    let mut scanner = String::new();

    io::stdin()
        .read_line(&mut scanner)
        .expect("Something went wrong when reading user input");

    let num_1 = scanner.trim();

    match num_1.parse::<f64>() {
        Ok(i) => {}
        Err(e) => {
            println!("This is not a number {}", num_1);
            println!("Error: {}", e.to_string());
        }
    }

    scanner = String::new();
    io::stdin()
        .read_line(&mut scanner)
        .expect("Something went wrong when reading user input");
}

/*
fn main() {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => println!("your integer input: {}", i),
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}

*/
