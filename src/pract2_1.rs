//Very ugly styling XD
//Exit only works on Linux ATM
use std::io;
use std::f64;
use std::process;

fn main() {

    let mut entrada = String::new();
    println!("Ingrese el valor del ángulo que desea");
    io::stdin().read_line(&mut entrada).unwrap();

    let angulo: f64 = entrada.trim().parse().unwrap();

    if angulo.cos() != 0.0 {
        let tang:f64 = angulo.sin() / angulo.cos();
        println!("La tangente del ángulo es: {}", tang);
        if angulo.sin() != 0.0 {
            let cot:f64 = angulo.cos() / angulo.sin();
            println!("La cotangente del ángulo es: {}", cot);
        } else {
            println!("El seno del ángulo debe ser diferente de 0");
            process::exit(1);
        }
    } else {
        println!("El coseno del ángulo debe de ser diferente de 0");
        process::exit(1);
    }
}
