use std::io;

fn main() {
    let mut input = String::new();
    println!("Ingrese el sueldo del trabajador");
    io::stdin().read_line(&mut input).unwrap();

    let mut sueldo:f64 = input.trim().parse().unwrap();

    if sueldo < 1000.00 {
        sueldo = sueldo * 1.15;
        print!("El sueldo total del trabajador es: ${}", sueldo);
    } else {
        sueldo = sueldo * 1.12;
        print!("El sueldo total del trabajador es: ${}", sueldo);
    }
}
