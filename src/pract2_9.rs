use std::io;

fn main() {
    let mut input = String::new();
    println!("Input the first number");
    io::stdin().read_line(&mut input).unwrap();

    let num1:f64 = input.parse().unwrap();

    let mut input_1 = String::new();
    println!("Input the second number");
    io::stdin().read_line(&mut input_1).unwrap();

    let num2:f64 = input_1.parse().unwrap();

    let mut input_2 = String::new();
    println!("Input the third number");
    io::stdin().read_line(&mut input_2).unwrap();

    let num3:f64 = input_2.parse().unwrap();

    if num1 > num2 && num1 > num3 {
        println!("Number {} is bigger than {} y {}", num1, num2, num3);
    } else if num2 > num1 && num2 > num3 {
        println!("Number {} is bigger than {} y {}", num2, num1, num3);
    } else if num3 > num1 && num3 > num2 {
        println!("Number {} is bigger than {} y {}", num3, num1, num2);
    }

}

