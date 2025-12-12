use std::io::{self, Write};

fn main() {
    print!("Enter any number:");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to take input");

    let num: i32 = input.trim().parse().expect("please type a valid number");
    println!("The Entered number is {}", num);
}
