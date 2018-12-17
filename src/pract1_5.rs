use std::io;

pub fn main() {
    const DOLAR:f64 = 19.29;
    
    let mut input = String::new();
    println!("Input the amount of dollars you posses");
    io::stdin().read_line(&mut input).unwrap();

    let mxn:f64 = input.parse().unwrap();
    println!("You have ${} Mexican Pesos", mxn * DOLAR);
}
