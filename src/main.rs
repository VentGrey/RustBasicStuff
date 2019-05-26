use std::io;

fn main() {
    //-- MENU
    println!("Please select an exercise to execute:");
    println!("1 - Basic operations");

    //-- Menu processing
    let mut op = String::new();

}

fn basic() {
    println!("Ingrese el primer número (en formato flotante)");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Could not read from stdin");

    let num1: f64 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a number!"),
    };

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
