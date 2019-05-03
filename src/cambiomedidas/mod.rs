use std::io;

pub fn cambio() {
    println!("Ingrese la extensión de su campo en acres");

    let mut scanner = String::new();

    io::stdin()
        .read_line(&mut scanner)
        .expect("Error al leer desde teclado");

    const VALHECT: f64 = 0.404686;

    let acres = scanner.trim();
    let mut hectareas: f64 = 0.0;

    match acres.parse::<f64>() {
        Ok(acres) => {
            hectareas = acres * VALHECT;
        },
        Err(_e) => {
            eprintln!("No es un número válido {}", _e);
            panic!();
        }
    }
    println!("Usted posee {} hectáreas", hectareas);
}
