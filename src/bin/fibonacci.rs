use std::io;

fn main() {
    println!("Enter the index of the fibonacci series: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read the input");

    let num: i64 = input.trim().parse().expect("Enter a valid index");

    let mut num1 = 0;
    let mut num2 = 1;

    if num == 1 {
        println!("0");
        return;
    } else if num == 2 {
        println!("1");
        return;
    }

    for _i in 0..num - 2 {
        let temp = num1 + num2;
        num1 = num2;
        num2 = temp;
    }

    println!("The {} number of the fibonacci series is {}", num, num2);
}
