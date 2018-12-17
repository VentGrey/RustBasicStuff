use std::io;

fn main() {
    let mut input = String::new();
    println!("Input your money import");
    io::stdin().read_line(&mut input).unwrap();

    let compra: f64 = input.trim().parse().unwrap();

    if compra > 2500.00 {
        let total:f64 = compra - (compra * 0.08);
        println!("You have to pay: ${}", total);
    } else {
        println!("You have to pay: ${}", compra);
    }
}
