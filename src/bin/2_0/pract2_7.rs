/*
Divisor

Elabore un programa que pueda determinar, dados dos números enteros, si
un número es divisor de otro.

Datos
NUM1, NUM2 variables de tipo entero.
*/

use std::io;

fn main() {
        let mut entrada = String::new();
        println!("Ingrese el primer número");
        io::stdin().read_line(&mut entrada).unwrap();

        let num1:i32 = entrada.parse().unwrap();

        let mut entrada_2 = String::new();
	println!("Ingrese el segundo número");
        io::stdin().read_line(&mut entrada_2).unwrap();

        let num2:i32 = entrada_2.parse().unwrap();

        if num1 % num2 == 0.0 {
                println!("Los números son divisibles");
        } else {
            panic!("Los numeros no son divisibles");
        }
}
