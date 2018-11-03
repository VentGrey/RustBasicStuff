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
    let mut entrada = String::new();
    println!("Ingrese la cantidad de unidades que desea convertir");
    io::stdin().read_line(&mut entrada).unwrap();

    let med:f64 = entrada.parse().unwrap();

    let mut entrada_1 = String::new();
    println!("Ingrese la conversión deseada: ");
    println!("1° Pulgadas a Milímetros");
    println!("2° Yardas a Metros");
    println!("3° Millas a Kilómetros");
    io::stdin().read_line(&mut entrada_1).unwrap();

    let op:i8 = entrada_1.parse().unwrap();

    match op {
        1 => println!("{} Pulgadas equivalen a {} Milímetros", med, med * 25.40),
        2 => println!("{} Yardas equivalen a {} Metros", med, med * 0.9144),
        3 => println!("{} Millas equivalen a {} Kilómetros", med, med * 1.6093),
        _ => panic!("ingrese un operador válido!"),
    }
}
