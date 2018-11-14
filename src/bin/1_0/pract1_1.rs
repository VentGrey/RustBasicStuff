use std::io;

fn main() {
    //Procesamiento de la primer variable
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
