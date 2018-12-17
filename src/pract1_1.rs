/* Write a program that takes two numbers from the user input
and calculates the addition, substraction and division of both
numbers */

use std::io;

fn main() {
    //First Variable Processind
    let mut input = String::new();
    println!("Enter the first number");

    io::stdin().read_line(&mut input).unwrap();
    let num1: f64 = input.trim().parse().unwrap();

    //Procesamiento de la segunda variable
    let mut input_2 = String::new();
    println!("\nEnter the second number");
    io::stdin().read_line(&mut input_2).unwrap();

    let num2: f64 = input_2.trim().parse().unwrap();

    println!("The sum of the previous numbers is: {}", num1 + num2);
    println!("The product of the previous numbers is: {}", num1 * num2);
    println!("The division of the previous numbers results in: {}", num1 / num2);
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