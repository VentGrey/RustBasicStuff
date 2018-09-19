use std::io;

fn main() {
    println!("*****------------------*****");
    println!("*");
    let mut entrada = String::new();
    println!("* Ingrese los km del viaje *");
    io::stdin().read_line(&mut entrada).unwrap();

}
