use std::io;

fn main() {
    let mut input = String::new();
    println!("Input the first number");
    io::stdin().read_line(&mut input).unwrap();

    let mut input_1 = String::new();
    println!("Input the second number");
    io::stdin().read_line(&mut input_1).unwrap();

    let num1:f64 = input.parse().unwrap();
    let num2:f64 = input_1.parse().unwrap();

    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2).unwrap();

    let operador:char = input.parse().unwrap();

    match operador {
        '+' => println!("The sum of the numbers is: {}", num1 + num2),
        '-' => println!("The diff of the numbers is: {}", num1 - num2),
        '*' => println!("The product of the numbers is: {}", num1 * num2),
        '/' => println!("The division of the numbers is: {}", num1 / num2),
        _ => println!("Input a valid operator or else you will feel the pain!"),
    }
}
