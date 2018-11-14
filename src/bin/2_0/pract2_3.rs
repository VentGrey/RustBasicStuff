use std::io;

fn main() {
    let mut input = String::new();
    println!("Input your worker's salary");
    io::stdin().read_line(&mut input).unwrap();

    let mut sueldo:f64 = input.trim().parse().unwrap();

    if sueldo < 1000.00 {
        sueldo = sueldo * 1.15;
        print!("Your workers salary now is: ${}", sueldo);
    } else {
        sueldo = sueldo * 1.12;
        print!("Your workers salary now is: ${}", sueldo);
    }
}
