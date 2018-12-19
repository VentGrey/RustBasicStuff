use std::io;

pub fn main() {
    const P_KM: f64 = 0.23;

    let mut input = String::new();
    println!("* Input your trip kilometers *");
    io::stdin().read_line(&mut input).unwrap();

    let mut km: f64 = input.parse().unwrap();

    let mut input_1 = String::new();
    println!("* How long will you stay? (in days) *");
    io::stdin().read_line(&mut input_1).unwrap();

    let days: i32 = input_1.parse().unwrap();

    if days <= 0 || km <= 0.0 {
        panic!("._. Not funny");
    } else if days > 7 && km > 800.00 {
        km = km * P_KM;
        km = km - (km * 0.30);
        println!("Your total is: ${}", km);
    }

    km = km * P_KM;
    println!("Your total is: ${}", km);
}
