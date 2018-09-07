use std::io;

fn main() {
    let mut entrada = String::new();
    println!("Ingrese la cantidad de su compra");
    io::stdin().read_line(&mut entrada).unwrap();

    let compra: f64 = entrada.trim().parse().unwrap();

    if compra > 2500.00 {
        let total:f64 = compra - (compra * 0.08);
        println!("Su total de compra es: {}", total);
    } else {
        println!("El total de su compra es: {}", compra);
    }
}
