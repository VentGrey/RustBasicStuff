mod basic;
mod circle;
mod divisas;
mod cambiomedidas;

use std::io;

fn main() {
    println!("----------Menú principal----------");
    println!("Seleccione el código a ejecutar:");
    println!("1- Operaciones básicas");
    println!("2- Área y perímetro del círculo");
    println!("3- Cambio de divisas");
    println!("4- Cambio de medidas");

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    let num: u32 = entrada.trim().parse().unwrap();

    match num {
        1 => basic::basic_op(),
        2 => circle::circle(),
        3 => divisas::divisas(),
        4 => cambiomedidas::cambio(),
        _ => panic!("Error de entrada D:"),
    }
}
