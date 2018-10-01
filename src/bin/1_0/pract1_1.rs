use std::io;

fn main() {
    //Procesamiento de la primer variable
    let mut entrada = String::new();
    println!("Ingrese el primer numero");

    io::stdin().read_line(&mut entrada).unwrap();
    let num1: f64 = entrada.trim().parse().unwrap();

    //Procesamiento de la segunda variable
    let mut entrada2 = String::new();
    println!("\nIngrese el segundo número");
    io::stdin().read_line(&mut entrada2).unwrap();

    let num2: f64 = entrada2.trim().parse().unwrap();

    println!("La suma de los números es: {}", num1 + num2);
    println!("La multiplicación de los números es: {}", num1 * num2);
    println!("La división de los números es: {}", num1 / num2);
}
