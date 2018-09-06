use std::io;

fn main() {
    let mut entrada = String::new();
    println!("Ingrese la base del triángulo / lado del pentágono");
    io::stdin().read_line(&mut entrada).unwrap();

    //Variable de asignación
    let base: f64 = entrada.trim().parse().unwrap();

    let mut entrada2 = String::new();
    println!("Ingrese la altura del triángulo / aporema del pentágono");
    io::stdin().read_line(&mut entrada2).unwrap();

    //Variable de asignación
    let altura: f64 = entrada2.trim().parse().unwrap();

    //Hacer las operaciones de las prácticas
    println!("El área del triángulo es: {}", (base * altura) / 2.0);
    println!("El área del pentágono es: {}", (base * 5.0) * altura);
}
