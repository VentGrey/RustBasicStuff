//---Modules spam, brace yourselves!
//-- 1st Part Modules
mod pract1_1;


use std::io;

fn main() {
    println!("----------Menu-------------------------------------------------");
    println!("Welcome to the main menu");
    println!("---------------------------------------------------------------");
    println!("These are the current programs in the project, status are\
    reported at the right");
    println!("Please select your desired practices folder:");

    let mut scanner = String::new();
    io::stdin()
        .read_line(&mut scanner)
        .expect("Failed to read from stdin library");

    let option = scanner.trim();
    match option.parse::<i8>() {
        Ok(i) => {

        },
        Err(e) => {
            println!("Error: {}", e.to_string());
        },
    }
}
