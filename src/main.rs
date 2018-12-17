//---Modules spam, brace yourselves!
//-- 1st Part Modules
mod pract1_1;
mod pract1_2;
mod pract1_3;
mod pract1_4;
mod pract1_5;

use std::io;

fn main() {
    println!("----------Menu-------------------------------------------------");
    println!("Welcome to the main menu");
    println!("---------------------------------------------------------------");
    println!(
        "These are the current programs in the project, status are\
         reported at the right"
    );
    println!("------FIRST UNIT-----");
    println!("(1) 1.1 => Basic aritmethics [IN PROGRESS]");
    println!("Please select your desired practice to execute:");

    let mut usr_input = String::new();
    io::stdin()
        .read_line(&mut usr_input)
        .expect("Failed to read from stdin library");

    let option = usr_input.trim();
    match option.parse::<i8>() {
        Ok(i) => {}
        Err(e) => {
            println!("This is not an integer {}", option);
            println!("Specific error report:");
            println!("Error: {}", e.to_string());
        }
    }
}
