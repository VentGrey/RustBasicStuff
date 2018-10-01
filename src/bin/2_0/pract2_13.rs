use std::io;

fn main() {
    let mut entrada = String::new();

    println!("Ingrese la clave de región a donde desea llamar");
    println!("12 - - América del Norte");
    println!("15 - - América Central" );
    println!("18 - - América del Sur");
    println!("19 - - Europa");
    println!("23 - - Asia");
    println!("25 - - Africa");
    println!("29 - - Oceanía");
    io::stdin().read_line(&mut entrada).unwrap();

    let clave:i16 = entrada.parse().unwrap();

    let mut entrada2 = String::new();
    println!("Ingrese el número de minutos de la llamada");
    io::stdin().read_line(&mut entrada2).unwrap();

    let mins:i32 = entrada2.parse().unwrap();

    match clave {
        12 => {
            if mins >= 3 {
                
            }
        }
    }
}
