use std::io;

fn main() {
    let mut input = String::new();
    println!("Input your worker's salary");
    io::stdin().read_line(&mut input).unwrap();

    let mut salary:f64 = input.trim().parse().unwrap();

    if salary < 1000.00 {
        salary = salary * 1.15;
        print!("Your workers salary now is: ${}", salary);
    } else {
        salary = salary * 1.12;
        print!("Your workers salary now is: ${}", salary);
    }
}
