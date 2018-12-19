//---External crates
extern crate colored;
//---Modules spam, brace yourselves!
//-- 1st Part Modules
mod pract1_1;
mod pract1_2;
mod pract1_3;
mod pract1_4;
mod pract1_5;
//-- 2nd Part Modules
mod pract2_1;
mod pract2_10;
mod pract2_11;
mod pract2_12;
mod pract2_13;
mod pract2_14;
mod pract2_2;
mod pract2_3;
mod pract2_4;
mod pract2_5;
mod pract2_6;
mod pract2_7;
mod pract2_8;
mod pract2_9;
//-- 3rd Part Modules
mod pract3_1;
//-- 4th Part Modules
//-- Extra Modules
//-- Keep learning modules

//-- External imports
use colored::*; // <-- Namespace violation, don't do this please

//-- Standard imports
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
    println!("(1) 1.1 => Basic aritmethics\
     [{}]", "IN PROGRESS".yellow());
    println!("(2) 1.2 => Repeat your name 4 times [{}]", "INCOMPLETE".red());
    println!("(3) 1.3 => Triangle/Pentagon formulas [{}]", "INCOMPLETE".red());
    println!("(4) 1.4 => Circle formula [{}]", "INCOMPLETE".red());
    println!("(5) 1.5 => Currency exchange [{}]", "INCOMPLETE".red());
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
    println!("Please select your desired practice to execute:");

    let mut usr_input = String::new();
    io::stdin()
        .read_line(&mut usr_input)
        .expect("Failed to read from stdin library");

    let option:i32 = usr_input.trim().parse().unwrap();

    match option {
        1 => pract1_1::main(),
        2 => pract1_2::main(),
        3 => pract1_3::main(),
        4 => pract1_4::main(),
        5 => pract1_5::main(),
        6 => pract2_1::main(),
        7 => pract2_2::main(),
        8 => pract2_3::main(),
        9 => pract2_4::main(),
        10 => pract2_5::main(),
        11 => pract2_6::main(),
        12 => pract2_7::main(),
        13 => pract2_8::main(),
        14 => pract2_9::main(),
        15 => pract2_10::main(),
        16 => pract2_11::main(),
        17 => pract2_12::main(),
        18 => pract2_13::main(),
        19 => pract2_14::main(),
        20 => pract3_1::main(),
        _ => panic!("Input error!"),
    }
}
