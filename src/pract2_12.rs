use std::io;

pub fn main() {
    let mut input = String::new();
    println!("Enter the amount of units you wish to convert");
    io::stdin().read_line(&mut input).unwrap();

    let med:f64 = input.parse().unwrap();

    let mut input_1 = String::new();
    println!("Input your conversion option: ");
    println!("1° Inches to Milimeters");
    println!("2° Yards to Meters");
    println!("3° Miles to Kilometers");
    io::stdin().read_line(&mut input_1).unwrap();

    let op:i8 = input_1.parse().unwrap();

    match op {
        1 => println!("{} Inches are {} Milimeters", med, med * 25.40),
        2 => println!("{} Yards are {} Meters", med, med * 0.9144),
        3 => println!("{} Miles are {} Kilometers", med, med * 1.6093),
        _ => panic!("Please insert a valid operator!"),
    }
}
