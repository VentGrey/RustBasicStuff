use std::io;

fn main() {
    //Procesamiento de la primer variable
    let mut input = String::new();
    println!("Ingrese el primer numero");

    io::stdin().read_line(&mut input).unwrap();
    let num1: f64 = input.trim().parse().unwrap();

    //Procesamiento de la segunda variable
    let mut input_2 = String::new();
    println!("\nIngrese el segundo número");
    io::stdin().read_line(&mut input_2).unwrap();

    let num2: f64 = input_2.trim().parse().unwrap();

    println!("La suma de los números es: {}", num1 + num2);
    println!("La multiplicación de los números es: {}", num1 * num2);
    println!("La división de los números es: {}", num1 / num2);
}
