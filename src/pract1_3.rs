extern crate colored;

use colored::*;
use std::io;

pub fn main() {
    println!("{}", "Triangle/Pentagon surface calculator!".yellow());

    loop {
        println!("Input the triangle's base / pentagon's height");
        let mut base = String::new();

        io::stdin()
            .read_line(&mut base)
            .expect("Failed to read from stdin");

        let base: f64 = match base.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut base2 = String::new();
        io::stdin()
            .read_line(&mut base2)
            .expect("Failed to read from stdin");

        let base2: f64 = match base2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}

/*
     let mut input = String::new();
    println!("Input the triangle's base / Pentagon's height");
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read from stdin");

    //Variable de asignación
    let base: f64 = input.trim().parse().unwrap();

    let mut input_2 = String::new();
    println!("Input the triangle's height / penthagon apothem");
    io::stdin().read_line(&mut input_2).unwrap();

    //Variable de asignación
    let height: f64 = input_2.trim().parse().unwrap();

    //Hacer las operaciones de las prácticas
    println!("The triangle's area is: {}", (base * height) / 2.0);
    println!("The pentagon's area is: {}", (base * 5.0) * height);
}*/
