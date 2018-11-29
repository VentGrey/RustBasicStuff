/*
 * Elabore un programa que permita convertir de pulgadas a milı́metros, de
 * yardas a metros y de millas a kilómetros.

 * Datos:
   OPCION es una variable de tipo entero que representa el tipo de conversión
   que se desea realizar.

   Se ingresa:
   1. Para convertir de pulgadas a milı́metros.
   2. Para convertir de yardas a metros.
   3. Para convertir de millas a kilómetros.

   Variables:
   MED es una variable de tipo real que representa la medida que se ingresará pa-
   ra conversión.

Consideraciones
1 pulgada equivale a 25.40 milímetros.
1 yarda equivale a 0.9144 metros.
1 milla equivale a 1.6093 kilómetros.
*/

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter the amount of units you wish to convert");
    io::stdin().read_line(&mut input).unwrap();

    let med:f64 = input.parse().unwrap();

    let mut input_1 = String::new();
    println!("Input your conversion option: ");
    println!("1° Inches to Milimeters");
    println!("2° Yards to Meters");
    println!("3° Miles to Kilometers");
    io::stdin().read_line(&mut input_1).unwrap();

    let op:i8 = input_1.parse().unwrap();

    match op {
        1 => println!("{} Inches are {} Milimeters", med, med * 25.40),
        2 => println!("{} Yards are {} Meters", med, med * 0.9144),
        3 => println!("{} Miles are {} Kilometers", med, med * 1.6093),
        _ => panic!("Please insert a valid operator!"),
    }
}
