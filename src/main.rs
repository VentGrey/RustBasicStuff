//---Modules spam, brace yourselves!
//-- 1st Part Modules
mod pract1_1;
mod pract1_2;
mod pract1_3;
mod pract1_4;
mod pract1_5;
//-- 2nd Part Modules
mod pract2_1;
mod pract2_2;
mod pract2_3;
mod pract2_4;
mod pract2_5;
mod pract2_6;
mod pract2_7;
mod pract2_8;
mod pract2_9;
mod pract2_10;
mod pract2_11;
mod pract2_12;
mod pract2_13;
mod pract2_14;
//-- 3rd Part Modules
mod pract3_1;


use std::io;

fn main() {
    println!("----------Menu-------------------------------------------------");
    println!("Welcome to the main menu");
    println!("---------------------------------------------------------------");
    println!(
        "These are the current programs in the project, status are\
         reported at the right"
    );
    //TODO: Add excecise names
    println!("------FIRST UNIT-----");
    println!("(1) 1.1 => Basic aritmethics [IN PROGRESS]");
    println!("(2) 1.2 => Repeat your name 4 times [INCOMPLETE]");
    println!("(3) 1.3 => Triangle/Pentagon formulas [INCOMPLETE]");
    println!("(4) 1.4 => Circle formula [INCOMPLETE]");
    println!("(5) 1.5 => Currency exchange [INCOMPLETE]");
    println!("------SECOND UNIT----");
    println!("(6) 2.1 => Angle processing [INCOMPLETE]");
    println!("(7) 2.2 => Basic money discount [INCOMPLETE]");
    println!("(8) 2.3 => Basic worker's salary processing [INCOMPLETE]");
    println!("(9) 2.4 => Floating point in fraction notation [INCOMPLETE]");
    println!("(10) 2.5 => Basic train ticket system [INCOMPLETE]");
    println!("(11) 2.6 => Worker salary conditionals [INCOMPLETE]");
    println!("(12) 2.7 => Are these two numbers divisible? [INCOMPLETE]");
    println!("(13) 2.8 => Sports practice suggestions [INCOMPLETE]");
    println!("(14) 2.9 => The biggest of three numbers [INCOMPLETE]");
    println!("(15) 2.10 => Advanced aritmethics [INCOMPLETE]");
    println!("(16) 2.11 => Car insurance payments [INCOMPLETE]");
    println!("(17) 2.12 => Distance conversion [INCOMPLETE]");
    println!("(18) 2.13 => Call center bills management [INCOMPLETE]");
    println!("(19) 2.14 => Hospital payment [INCOMPLETE]");
    println!("------THIRD UNIT-----");
    println!("(20) 3.1 => Consecutive sum of numbers [INCOMPLETE]");
    println!("------FOURTH UNIT----");
    println!("------EXTRAS---------");
    println!("----KEEP LEARNING----");
    //TODO: Add the whole sections
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
