use std::io;

fn main() {
    const PI:f64 = 3.14159;
    let mut input = String::new();
    println!("Input the circle radius");
    io::stdin().read_line(&mut input).unwrap();

    //Procesar el círculo
    let radius:f64 = input.trim().parse().unwrap();

    println!("Area of this circle is: {}", PI * radius * radius);
    println!("Circumference is equal to: {}", PI * (radius * 2.0));
}
