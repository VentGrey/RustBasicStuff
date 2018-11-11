use std::io;

fn main() {
    let mut input = String::new();

    println!("Ingrese la clave de región a donde desea llamar");
    println!("12 - - América del Norte");
    println!("15 - - América Central" );
    println!("18 - - América del Sur");
    println!("19 - - Europa");
    println!("23 - - Asia");
    println!("25 - - Africa");
    println!("29 - - Oceanía");
    io::stdin().read_line(&mut input).unwrap();
    let clave:i16 = input.parse().unwrap();

    let mut input_2 = String::new();
    println!("Ingrese el número de minutos de la llamada");
    io::stdin().read_line(&mut input_2).unwrap();

    let mins:f64 = input_2.parse().unwrap();
    let mut precio:f64 = 0.0;

    //Desmadre de llaves incoming
    //Brace yourselves!
    //
    //MAS DE 3 NIVELES DE INDENT! El código está mal :(
    match clave {
        12 => {
                if mins >= 3.0 {
                    precio = 2.0;
                    precio = precio * mins;
                    println!("Su importe total es de: ${} pesos", precio);
                } else if mins <= 4.0 {
                    precio = 1.5;
                    precio = precio * mins;
                    println!("Su importe total es de: ${} pesos", precio);
                } else {
                    panic!("Ocurrió un eror!");
                }
        }
        15 => {
                if mins >= 3.0 {
                    precio = 2.2;
                    precio = precio * mins;
                    println!("Su importe total es de: ${} pesos", precio);
                } else if mins <= 4.0 {
                    precio = 1.8;
                    precio = precio * mins;
                    println!("Su importe total es de: ${} pesos", precio);
                } else {
                    panic!("Ocurrió un eror!");
                }
        }
        18 => {
                if mins >= 3.0 {
                    precio = 4.5;
                    precio = precio * mins;
                    println!("Su importe total es de: ${} pesos", precio);
                } else if mins <= 4.0 {
                    precio = 3.5;
                    precio = precio * mins;
                    println!("Su importe total es de: ${} pesos", precio);
                } else {
                    panic!("Ocurrió un eror!");
                }
        }
        19 => {
                if mins >= 3.0 {
                    precio = 3.5;
                    precio = precio * mins;
                    println!("Su importe total es de: ${} pesos", precio);
                } else if mins <= 4.0 {
                    precio = 2.7;
                    precio = precio * mins;
                    println!("Su importe total es de: ${} pesos", precio);
                } else {
                    panic!("Ocurrió un eror!");
                }
        }
        23 | 25 => {
                if mins >= 3.0 {
                    precio = 6.0;
                    precio = precio * mins;
                    println!("Su importe total es de: ${} pesos", precio);
                } else if mins <= 4.0 {
                    precio = 4.6;
                    precio = precio * mins;
                    println!("Su importe total es de: ${} pesos", precio);
                } else {
                    panic!("Ocurrió un eror!");
                }
        }
        29 => {
                if mins >= 3.0 {
                    precio = 5.0;
                    precio = precio * mins;
                    println!("Su importe total es de: ${} pesos", precio);
                } else if mins <= 4.0 {
                    precio = 3.9;
                    precio = precio * mins;
                    println!("Su importe total es de: ${} pesos", precio);
                } else {
                    panic!("Ocurrió un eror!");
                }
        }
        _ => panic!("Ingrese una opción válida!");

    }
}
