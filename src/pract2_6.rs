use colored::*;
use std::io;

pub fn main() {
    let mut input = String::new();
    println!("Input worker salary");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from STDIN");

    let mut salary: f64 = input.trim().parse().unwrap();

    if salary < 10000.00 {
        salary *= 1.15;
        println!("Your total salary is {}", salary);
    } else if salary >= 10000.00 {
        salary *= 1.11;
        println!("Your total salary is {}", salary);
    } else if salary > 15000.00 {
        salary *= 1.08;
        println!("Your total salary is {}", salary);
    } else {
        panic!("._. You can get sued if you don't pay your workers");
    }
}
