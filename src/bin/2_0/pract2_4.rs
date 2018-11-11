use std::io;
use std::i32;

fn main() {
    //Entrada 0
    let mut input = String::new();
    println!("Ingrese el dato A");
    io::stdin().read_line(&mut input).unwrap();

    let a:i32 = input.parse().unwrap();

    //Entrada 1
    let mut input_1 = String::new();
    println!("Ingrese el dato B");
    io::stdin().read_line(&mut input_1).unwrap();

    let b:i32 = input1.parse().unwrap();

    //Entrada 2
    let mut input_2 = String::new();
    println!("Ingrese el dato C");
    io::stdin().read_line(&mut input_2).unwrap();

    let c:i32 = input_2.parse().unwrap();

    //Entrada 3
    let mut input_3 = String::new();
    println!("Ingrese el dato D");
    io::stdin().read_line(&mut input_3).unwrap();

    let d:i32 = input_3.parse().unwrap();

    //Procesar todo, otra vez.

    if d == 0 {
        panic!("El divisor no puede ser cero!");
    }

    println!("El resultado de la primera fracción es: {}",( ((a - c, 2).pow(2)) / d));
    println!("El resultado de la segunda fracción es: {}",( ((a - b, 2) ).pow(2) / d));


}
