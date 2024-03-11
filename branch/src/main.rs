use std::io;

fn main() {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let number:isize = number.trim().parse().expect("not a valid input");
    if number<5{
        println!("true");
    }
    else{
        println!("false");
    }
}
