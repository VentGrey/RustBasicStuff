// Ask the user for his/her name and print it 3 times
use std::io;

pub fn main() {
    let mut input = String::new();
    println!("Enter your first name\n");
    io::stdin().read_line(&mut input).unwrap();

    for _i in 0..4 {
        println!("\n{}", input);
    }
}
