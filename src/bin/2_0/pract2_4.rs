use std::io;
use std::i32;

fn main() {
    //Entrada 0
    let mut entrada = String::new();
    println!("Ingrese el dato A");
    io::stdin().read_line(&mut entrada).unwrap();

    let a:i32 = entrada.parse().unwrap();

    //Entrada 1
    let mut entrada1 = String::new();
    println!("Ingrese el dato B");
    io::stdin().read_line(&mut entrada1).unwrap();

    let b:i32 = entrada1.parse().unwrap();

    //Entrada 2
    let mut entrada2 = String::new();
    println!("Ingrese el dato C");
    io::stdin().read_line(&mut entrada2).unwrap();

    let c:i32 = entrada2.parse().unwrap();

    //Entrada 3
    let mut entrada3 = String::new();
    println!("Ingrese el dato D");
    io::stdin().read_line(&mut entrada3).unwrap();

    let d:i32 = entrada3.parse().unwrap();

    //Procesar todo, otra vez.

    if d == 0 {
        panic!("El divisor no puede ser cero!");
    }

    println!("El resultado de la primera fracción es: {}",( ((a - c, 2).pow(2)) / d));
    println!("El resultado de la segunda fracción es: {}",( ((a - b, 2) ).pow(2) / d));


}
