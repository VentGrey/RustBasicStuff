use std::io;

fn main() {
    let mut input = String::new();
    println!("Ingrese su nombre\n")
    io::stdin().read_line(&mut input).unwrap();


    for _i in 0..4 {
        println!("\n{}", input);
    }
}
