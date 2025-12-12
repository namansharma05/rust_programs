use std::io;

struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> i32 {
        return 2 * self.width * self.height;
    }

    fn _debug() -> i32 {
        // this function cannot be called explicitely as it does not have a "&self" argument
        return 1;
    }
}

fn main() {
    println!("Enter the width of the Rectangle: ");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("failed to read input1");

    let width: i32 = input1
        .trim()
        .parse()
        .expect("Enter the valid width of the rectangle");

    println!("Enter the height of the Rectangle: ");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("failed to read input2");

    let height: i32 = input2
        .trim()
        .parse()
        .expect("Enter the valid height of the rectangle");

    let rectangle = Rect { width, height };

    println!("Area of the Rectangle is {}", rectangle.area());
    println!("Perimeter of the Rectangle is {}", rectangle.perimeter());
}
