use std::io;

fn main() {
    let mut input = String::new();
    println!("Input the actual temperature");
    io::stdin().read_line(&mut input).unwrap();

    let temp:f64 = input.parse().unwrap();

    if temp > 85.00 {
        println!("You should go swimming");
    } else if temp > 70.00 && temp <= 85.00 {
        println!("Take a tennis match!");
    } else if temp > 32.00 && temp <= 70.00 {
        println!("Go practice some golf");
    } else if temp > 10.00 && temp <= 32.00 {
        println!("Ski in the mountains");
    } else if temp <= 10.00 {
        println!("Go take a trip");
    }
}
