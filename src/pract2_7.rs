use std::io;

fn main() {
        let mut input = String::new();
        println!("Input your first number");
        io::stdin().read_line(&mut input).unwrap();

        let num1:i32 = input.parse().unwrap();

        let mut input_2 = String::new();
	println!("Input your second number");
        io::stdin().read_line(&mut input_2).unwrap();

        let num2:i32 = input_2.parse().unwrap();

        if num1 % num2 == 0 {
                println!("Numbers can be divided");
        } else {
            panic!("Numbers cannot be divided");
        }
}
