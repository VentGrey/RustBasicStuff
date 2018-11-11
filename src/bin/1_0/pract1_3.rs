use std::io;

fn main() {
    let mut input = String::new();
    println!("Ingrese la base del triángulo / lado del pentágono");
    io::stdin().read_line(&mut input).unwrap();

    //Variable de asignación
    let base: f64 = input.trim().parse().unwrap();

    let mut input_2 = String::new();
    println!("Ingrese la altura del triángulo / aporema del pentágono");
    io::stdin().read_line(&mut input_2).unwrap();

    //Variable de asignación
    let altura: f64 = input_2.trim().parse().unwrap();

    //Hacer las operaciones de las prácticas
    println!("El área del triángulo es: {}", (base * altura) / 2.0);
    println!("El área del pentágono es: {}", (base * 5.0) * altura);
}
