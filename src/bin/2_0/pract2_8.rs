/* Elabore un programa que dado como dato una temperatura en grados Fah-
*  renheit, determine el deporte que es apropiado practicar a esa temperatura,
*  teniendo en cuenta la siguiente tabla:

Tabla
---------------------------------
Natación |      Temperatura > 85|
Tenis    | 70 < Temperatura ≤ 85|
Golf     | 32 < Temperatura ≤ 70|
Esquí    | 10 < Temperatura ≤ 32|
Marcha   |      Temperatura ≤ 10|
---------------------------------
*/
use std::io;

fn main() {
    let mut entrada = String::new();
    println!("Ingrese la temperatura actual");
    io::stdin().read_line(&mut entrada).unwrap();

    let temperatura:f64 = entrada.parse().unwrap();

    if temperatura > 85.00 {
        println!("Deberías practicar natación");
    } else if temperatura > 70.00 && temperatura <= 85.00 {
        println!("Deberías practicar Tenis");
    } else if temperatura > 32.00 && temperatura <= 70.00 {
        println!("Deberias practicar golf");
    } else if temperatura > 10.00 && temperatura <= 32.00 {
        println!("Deberías practicar esquí");
    } else if temperatura <= 10.00 {
        println!("Deberías practicar marcha");
    }
}
