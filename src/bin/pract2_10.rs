/* Elabore un programa que permita realizar operaciones aritméticas ele men-
* tales, según la clave ingresada.
|Clave | Operación |
|  +   | suma
|  -   | resta
|  *   | multiplicación
|  /   | división
----------------- */

use std::io;

fn main() {
    let mut entrada = String::new();
    println!("Ingrese el primer número");
    io::stdin().read_line(&mut entrada).unwrap();

    let mut entrada1 = String::new();
    println!("Ingrese el segundo número");
    io::stdin().read_line(&mut entrada1).unwrap();

    let num1:f64 = entrada.parse().unwrap();
    let num2:f64 = entrada1.parse().unwrap();

    let mut entrada2 = String::new();
    io::stdin().read_line(&mut entrada2).unwrap();

    let operador:char = entrada.parse().unwrap();

    match operador {
        '+' => println!("La suma de los números es: {}", num1 + num2),
        '-' => println!("La diferencia de los números es: {}", num1 - num2),
        '*' => println!("El producto resultante es: {}", num1 * num2),
        '/' => println!("La división resultante es: {}", num1 / num2),
        _ => println!("Ingrese un operador válido"),
    }
}
