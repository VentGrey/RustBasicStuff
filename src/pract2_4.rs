use std::io;

pub fn main() {
    let mut input = String::new();
    println!("Input A");
    io::stdin().read_line(&mut input).unwrap();

    let a: i32 = input.parse().unwrap();

    let mut input_1 = String::new();
    println!("Input B");
    io::stdin().read_line(&mut input_1).unwrap();

    let b: i32 = input_1.parse().unwrap();

    let mut input_2 = String::new();
    println!("Input C");
    io::stdin().read_line(&mut input_2).unwrap();

    let c: i32 = input_2.parse().unwrap();

    let mut input_3 = String::new();
    println!("Input D");
    io::stdin().read_line(&mut input_3).unwrap();

    let d: i32 = input_3.parse().unwrap();

    if d == 0 {
        panic!("You cannot divide by zero!");
    }

    let mut result_1: i32 = (a - c).pow(2);
    result_1 /= 2;

    // FIXME: No way to apply power ATM
}
