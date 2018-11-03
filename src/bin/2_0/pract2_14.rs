/*
Los clientes de un hospital tienen una credencial en la que además de otra
información registra una categorı́a que depende de los ingresos económicos del
núcleo familiar del paciente (cliente). Teniendo en cuenta la categorı́a el hospital
les hace un descuento cuando tienen que pagar su cuenta, con base en la siguiente
tabla:

Categoría   Descuento
    1           35%
    2           22%
    3           15%
    4           5%

Observe que las categorías mayores a 4 no tienen descuento
*/
use std::io;

fn main() {
    let mut entrada = String::new();
    println!("Bienvenido al hospital, ingrese la categoría de su tarjeta");
    io::stdin().read_line(&mut entrada).unwrap();

    let credencial:i8 = entrada.parse().unwrap();

    let mut entrada_2 = String::new();
    println!("Ingrese el monto en tu ticket");
    io::stdin().read_line(&mut entrada_2).unwrap();

    let monto:f64 = entrada_2.parse().unwrap();

    match credencial {
        1 => println!("Su monto a pagar es de: ${}", monto - (monto * 0.35)),
        2 => println!("Su monto a pagar es de: ${}", monto - (monto * 0.22)),
        3 => println!("Su monto a pagar es de: ${}", monto - (monto * 0.15)),
        4 => println!("Su monto a pagar es de: ${}", monto - (monto * 0.05)),
        _ => println!("Usted no recibe descuento, pague {}", monto),
    }
}
