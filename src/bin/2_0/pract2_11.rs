/* Elabore un programa que dado como datos el modelo de un vehı́culo y su
* precio, determine el valor final que debe pagar el comprador. El concesionario
* está haciendo descuentos teniendo en cuenta el modelo, con base en la siguiente

Modelo       | Descuento
Blazer-Trail |   8%
Cavallier    |   5%
Chevy        |   6%
Opel-Astra   |   9%

*/
use std::io;

fn main() {
    let mut entrada = String::new();
    println!("Ingrese el precio del vehículo");
    io::stdin().read_line(&mut entrada).unwrap();

    let prec:f64 = entrada.parse().unwrap();


    let mut entrada1 = String::new();
    println!("Ingrese el tipo de vehículo que desea");
    println!("1° Blazer-Trail");
    println!("2° Cavallier");
    println!("3° Chevy");
    println!("4° Opel-Astra");
    io::stdin().read_line(&mut entrada1).unwrap();

    let op:u8 = entrada1.parse().unwrap();

    match op {
        1 => println!("El precio del vehículo es: ${}", prec - (prec * 0.08)),
        2 => println!("El precio del vehículo es: ${}", prec - (prec * 0.05)),
        3 => println!("El precio del vehículo es: ${}", prec - (prec * 0.06)),
        4 => println!("El precio del vehículo es: ${}", prec - (prec * 0.09)),
        _ => println!(">:V Ingrese una opción válida por favor!"),
    }

}
