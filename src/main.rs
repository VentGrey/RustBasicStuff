use std::io;
use std::f64::consts::PI;

fn main() {
    //-- MENU
    println!("Please select an exercise to execute:");
    println!("1 - Basic operations");
    println!("2 - Circle");

    //-- Menu processing
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");

    let option: u32 = input.trim().parse().unwrap();

    match option {
        1 => basic(),
        2 => circle(),
        _ => {
            println!("Option not available! Returning to main...");
            main();
        },
    }

}

fn basic() {
    println!("Please input a first number");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Could not read from stdin");

    let num1: f64 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number!"),
    };

    println!("Please input another number");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Could not read from stdin");

    let num2: f64 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number!"),
    };

    println!("The sum of the numbers equals to: {}", num1 + num2);
    println!("The diff of the numbers equals to: {}", num1 - num2);
    println!("The product of the numbers equals to: {}", num1 * num2);
    println!("The division of the numbers equals to: {}", num1 / num2);
}

fn circle() {
    println!("Please input your circle's radius");
    let mut rad = String::new();
    io::stdin()
        .read_line(&mut rad)
        .expect("Could not read from stdin");

    let rad: f64 = match rad.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number!"),
    };

    println!("The area of the circle is equal to: {}", rad * PI * PI);
    println!("The perimeter of the circle is equal to: {}", (2.0 * rad) * PI);
}
