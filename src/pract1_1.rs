/* Write a program that takes two numbers from the user input
and calculates the addition, substraction and division of both
numbers */

//TODO: Fix this user input methods

use std::io;

pub fn main() {
    let mut scanner = String::new();

    let mut sum: f64 = 0.0;

    io::stdin()
        .read_line(&mut scanner)
        .expect("Something went wrong when reading user input");

    let num_1 = scanner.trim();

    match num_1.parse::<f64>() {
        Ok(num_1) => {
            println!("Number 1 registered correctly! {}", num_1);
        }
        Err(e) => {
            eprintln!("This is not a number {}", num_1);
            eprintln!("Error: {}", e.to_string());
        }
    }

    let mut scanner = String::new();
    io::stdin()
        .read_line(&mut scanner)
        .expect("Something went wrong when reading user input");

    let num_2 = scanner.trim();

    match num_2.parse::<f64>() {
        Ok(i) => {
            println!("Number 2 registered correctly! {}", i);
        }
        Err(e) => {
            eprintln!("This is not a number {}", num_2);
            eprintln!("Error: {}", e);
        }
    }

    println!("The sum of {} and {} is: {}", num_1, num_2, sum);
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
