/*
Elabore un programa que dado como dato el sueldo de un trabajador, calcule
su aumento e imprima el nuevo sueldo del trabajador.

Datos

 SUELDO variable de tipo real que representa el sueldo del trabajador.

  Consideraciones
* SUELDO menor que $10,000 entonces AUMENTO del 15 %
* SUELDO mayor o igual que $10,000 y menor o igual que $15,000 entonces
* AUMENTO del 11 %
* SUELDO mayor que %15,000 entonces AUMENTO del 8 %
*/
use std::io;

fn main() {
    let mut entrada = String::new();
    println!("Ingrese el sueldo del trabajador");
    io::stdin().read_line(&mut entrada).unwrap();

    let mut sueldo:f64 = entrada.parse().unwrap();

    if sueldo < 10000.00 {
    	sueldo = sueldo * 1.15;
    } else if sueldo >= 10000.00 {
    	sueldo = sueldo * 1.11;
    } else if sueldo > 15000.00 {
    	sueldo = sueldo * 1.08;
    } else {
    	panic!("._. te pueden demandar por no pagarle a tus trabajadores");
    }
}
