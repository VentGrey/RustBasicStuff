use std::io;

fn main() {
    const DOLAR:f64 = 19.29;
    
    let mut input = String::new();
    println!("Ingrese la cantidad de dólares que posee");
    io::stdin().read_line(&mut input).unwrap();

    let mxn:f64 = input.parse().unwrap();
    println!("Usted posee ${} pesos mexicanos", mxn * DOLAR);
}
