use std::io;

fn main() {
    let mut entrada = String::new();
    println!("Ingrese su nombre\n")
    io::stdin().read_line(&mut entrada).unwrap();


    for _i in 0..4 {
        println!("\n{}", entrada);
    }
}
