use std::io;

fn main() {
    println!("Enter a string: ");
    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("failed to read string");

    let mut count = 0;
    for char in string.chars() {
        if char == '\n' {
            break;
        }
        count += 1;
    }

    println!("Length of the string is {}", count);
}
