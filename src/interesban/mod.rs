use std::io;

pub fn interes() {
    println!("Ingrese el monto de dinero que depositará");
    let mut scanner = String::new();
    io::stdin()
        .read_line(&mut scanner)
        .expect("Error al leer desde teclado");
    let md = scanner.trim();

    let mut scanner = String::new(); // Reutilizar variable
    io::stdin()
        .read_line(&mut scanner)
        .expect("Error al leer desde teclado");
    let tasa = scanner.trim();

    let mut total:f64 = 0.0;

    match md.parse::<f64>() {
        Ok(md) => {
            total = md;
        },
        Err(e) => {
            eprintln!("Esto no es un número {}", e);
        }
    };

    match tasa.parse::<f64>() {
        Ok(tasa) => {
            total = total + total;
            total = total / tasa;
        }
        Err(e) => {
            eprintln!("Esto no es un número {}", e);
        }
    }
    println!("El total que pagará es de {}", total);
}
