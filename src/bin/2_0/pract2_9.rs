/* Elabore un programa que dados tres números reales A, B y C, identifique
*  cuál es el mayor. Considere a fin de simplificar el problema que los números son
*  diferentes. */

use std::io;

fn main() {
    let mut input = String::new();
    println!("Ingrese el primer número");
    io::stdin().read_line(&mut input).unwrap();

    let num1:f64 = input.parse().unwrap()

    let mut input_1 = String::new();
    println!("Ingrese el segundo número");
    io::stdin().read_line(&mut input_1).unwrap();

    let num2:f64 = input_1.parse().unwrap()

    let mut input_2 = String::new();
    println!("Ingrese el tercer número");
    io::stdin().read_line(&mut input_2).unwrap();

    let num3:f64 = input_2.parse().unwrap()

    if num1 > num2 && num1 > num3 {
        println!("El número {} es mayor que {} y {}", num1, num2, num3);
    } else if num2 > num1 && num2 > num3 {
        println!("El número {} es mayor que {} y {}", num2, num1, num3);
    } else if num3 > num1 && num3 > num2 {
        println!("El numero {} es mayor que {} y {}", num3, num1, num2);
    }

}

