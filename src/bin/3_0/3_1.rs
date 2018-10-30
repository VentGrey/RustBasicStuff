/* Suma números
 * Elabore un programa que calcule e imprima la suma de los N primeros
 * números naturales.*/

use std::io;

fn main() {
    let mut entrada = String::new();
    println!("Ingrese la cantidad de números naturales que desea sumar: ");
    io::stdin().read_line(&mut entrada).unwrap();

    let n:i64 = entrada.trim().parse().unwrap();
    let mut sum:i64 = 0;

    for i in 0..n {
        sum = sum + i;
    }

    println!("La suma de los {} números naturales es: {}", n, sum);
}
