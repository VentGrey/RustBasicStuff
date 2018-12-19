use std::io;

pub fn main() {
    let mut input = String::new();
    println!("Input worker salary");
    io::stdin().read_line(&mut input).unwrap();

    let mut salary:f64 = input.parse().unwrap();

    if salary < 10000.00 {
    	salary *= 1.15;
    } else if salary >= 10000.00 {
    	salary *= 1.11;
    } else if salary > 15000.00 {
    	salary *= 1.08;
    } else {
    	panic!("._. You can get sued if you don't pay your workers");
    }
}
