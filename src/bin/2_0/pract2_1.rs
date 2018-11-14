//Very ugly styling XD
//Exit only works on Linux ATM
use std::io;
use std::f64;
use std::process;

fn main() {

    let mut input = String::new();
    println!("Input some angle value");
    io::stdin().read_line(&mut input).unwrap();

    let angle: f64 = input.trim().parse().unwrap();

    if angle.cos() != 0.0 {
        let tang:f64 = angulo.sin() / angulo.cos();
        println!("Tangent is equal to: {}", tang);

        if angle.sin() != 0.0 {
            let cot:f64 = angle.cos() / angle.sin();

        if angulo.sin() != 0.0 {
            let cot:f64 = angulo.cos() / angulo.sin();

            println!("Cotangent is equal to: {}", cot);
        } else {
            panic!("Angle sin cannot be zero >:c");

        }
    } else {
        panic!("Cosine must be different from zero >:c");

    }
}
