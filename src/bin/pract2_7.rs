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

	let num1:f64 = entrada.parse().unwrap();

	entrada = String::new();
	
}