use std::io;

fn main() {
    let mut input = String::new();
    println!("Ingrese la cantidad de su compra");
    io::stdin().read_line(&mut input).unwrap();

    let compra: f64 = input.trim().parse().unwrap();

    if compra > 2500.00 {
        let total:f64 = compra - (compra * 0.08);
        println!("Su total de compra es: {}", total);
    } else {
        println!("El total de su compra es: {}", compra);
    }
}
