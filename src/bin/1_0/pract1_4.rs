use std::io;

fn main() {
    const PI:f64 = 3.14159;
    let mut input = String::new();
    println!("Ingrese el radio del círculo");
    io::stdin().read_line(&mut input).unwrap();

    //Procesar el círculo
    let radio:f64 = input.trim().parse().unwrap();

    println!("El área del círculo es: {}", PI * radio * radio);
    println!("La circunferencia es igual a: {}", PI * (radio * 2.0));
}
