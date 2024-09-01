use std::io;

fn main() {
    println!("Enter a number:");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: i32 = num.trim().parse().expect("Invalid number");

    if num < 0 {
        panic!("Invalid number for fibonacci");
    }

    println!("The fibonacci num of {} is {}", num, fibonacci(num));
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
