use std::io;

fn main() {
    let mut input = String::new();
    println!("Input the vehicle's price");
    io::stdin().read_line(&mut input).unwrap();

    let prec:f64 = input.parse().unwrap();


    let mut input_1 = String::new();
    println!("Input the vehicle type");
    println!("1° Blazer-Trail");
    println!("2° Cavallier");
    println!("3° Chevy");
    println!("4° Opel-Astra");
    io::stdin().read_line(&mut input_1).unwrap();

    let op:u8 = input_1.parse().unwrap();

    match op {
        1 => println!("Total price is: ${}", prec - (prec * 0.08)),
        2 => println!("Total price is: ${}", prec - (prec * 0.05)),
        3 => println!("Total price is: ${}", prec - (prec * 0.06)),
        4 => println!("Total price is: ${}", prec - (prec * 0.09)),
        _ => println!("Input a valid option please!"),
    }

}
