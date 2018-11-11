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
    let mut input = String::new();
    println!("Ingrese el primer número");
    io::stdin().read_line(&mut input).unwrap();

    let mut input_1 = String::new();
    println!("Ingrese el segundo número");
    io::stdin().read_line(&mut input_1).unwrap();

    let num1:f64 = input.parse().unwrap();
    let num2:f64 = input_1.parse().unwrap();

    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2).unwrap();

    let operador:char = input.parse().unwrap();

    match operador {
        '+' => println!("La suma de los números es: {}", num1 + num2),
        '-' => println!("La diferencia de los números es: {}", num1 - num2),
        '*' => println!("El producto resultante es: {}", num1 * num2),
        '/' => println!("La división resultante es: {}", num1 / num2),
        _ => println!("Ingrese un operador válido"),
    }
}
