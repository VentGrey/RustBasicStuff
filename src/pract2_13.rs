use std::io;

fn main() {
    let mut input = String::new();

    println!("Insert the specified region code you wish to call");
    println!("12 - - North America");
    println!("15 - - Center America" );
    println!("18 - - South America");
    println!("19 - - Europe");
    println!("23 - - Asia");
    println!("25 - - Africa");
    println!("29 - - Australia");
    io::stdin().read_line(&mut input).unwrap();
    let clave:i16 = input.parse().unwrap();

    let mut input_2 = String::new();
    println!("Input your call minutes");
    io::stdin().read_line(&mut input_2).unwrap();

    let mins:f64 = input_2.parse().unwrap();
    let mut precio:f64 = 0.0;

    //Desmadre de llaves incoming
    //Brace yourselves!
    //
    match clave {
        12 => {
                if mins >= 3.0 {
                    precio = 2.0;
                    precio = precio * mins;
                    println!("You have to pay ${}", precio);
                } else if mins <= 4.0 {
                    precio = 1.5;
                    precio = precio * mins;
                    println!("You have to pay ${}", precio);
                } else {
                    panic!("error!");
                }
        }
        15 => {
                if mins >= 3.0 {
                    precio = 2.2;
                    precio = precio * mins;
                    println!("You have to pay ${}", precio);
                } else if mins <= 4.0 {
                    precio = 1.8;
                    precio = precio * mins;
                    println!("You have to pay ${}", precio);
                } else {
                    panic!("error!");
                }
        }
        18 => {
                if mins >= 3.0 {
                    precio = 4.5;
                    precio = precio * mins;
                    println!("You have to pay ${}", precio);
                } else if mins <= 4.0 {
                    precio = 3.5;
                    precio = precio * mins;
                    println!("You have to pay ${}", precio);
                } else {
                    panic!("Something is wrong!!");
                }
        }
        19 => {
                if mins >= 3.0 {
                    precio = 3.5;
                    precio = precio * mins;
                    println!("You have to pay ${}", precio);
                } else if mins <= 4.0 {
                    precio = 2.7;
                    precio = precio * mins;
                    println!("You have to pay ${}", precio);
                } else {
                    panic!("Something is wrong!!");
                }
        }
        23 | 25 => {
                if mins >= 3.0 {
                    precio = 6.0;
                    precio = precio * mins;
                    println!("You have to pay ${}", precio);
                } else if mins <= 4.0 {
                    precio = 4.6;
                    precio = precio * mins;
                    println!("You have to pay ${}", precio);
                } else {
                    panic!("Something is wrong!!");
                }
        }
        29 => {
                if mins >= 3.0 {
                    precio = 5.0;
                    precio = precio * mins;
                    println!("You have to pay ${}", precio);
                } else if mins <= 4.0 {
                    precio = 3.9;
                    precio *= mins;
                    println!("You have to pay ${}", precio);
                } else {
                    panic!("Something is wrong!!");
                }
        }
        _ => panic!("Invalid option"),

    }
}
