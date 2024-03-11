use std::io;
fn main() {
    let mut x = String::new();
    println!("enter the temperature in fahrenheit");
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read the line.");
    let x:f32 = x.trim().parse().expect("enter a valid input");
    let y:f32 = (x-32.0)*(5.0/9.0);
    println!("temperature in celcius is {y}");
}

