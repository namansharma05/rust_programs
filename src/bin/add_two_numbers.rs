use std::io;

fn main() {
    println!("Enter first number: ");
    let mut input1 = String::new();

    io::stdin()
        .read_line(&mut input1)
        .expect("failed to read first input1");
    let num1: i32 = input1.trim().parse().expect("Enter a valid first number");

    println!("Enter second number: ");
    let mut input2 = String::new();

    io::stdin()
        .read_line(&mut input2)
        .expect("failed to read the second input");
    let num2: i32 = input2.trim().parse().expect("Enter a valid second number");

    let sum = num1 + num2;
    println!("Sum of {} and {} is {}", num1, num2, sum);
}
