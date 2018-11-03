use std::io;

fn main() {
    const P_KM:f64 = 0.23;

    let mut entrada = String::new();
    println!("* Ingrese los km del viaje *");
    io::stdin().read_line(&mut entrada).unwrap();

    let mut km:f64 = entrada.parse().unwrap();

    let mut entrada_1 = String::new();
    println!("* Ingrese el tiempo de estancia *");
    io::stdin().read_line(&mut entrada_1).unwrap();

    let dias:i32 = entrada_1.parse().unwrap();

    if dias <= 0 || km <=  0.0 {
        panic!("._. ¿entonces para que quieres viajar?");
    } else if dias > 7 && km > 800.00 {
        km = km * P_KM;
        km = km - (km * 0.30);
        println!("Su precio total a pagar es de: ${}", km);
    }

    km = km * P_KM;
    println!("Su precio a pagar es de ${}", km);


}
