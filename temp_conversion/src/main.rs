fn main() {
    let x = convert_to_fahrenheit(100);
    println!("The temperature in fahrenheit is {x}F");

    let y = convert_to_celsius(212);
    println!("The temperature in celsius is {y}C");
}

fn convert_to_fahrenheit(temp: i32) -> i32 {
    (temp * 9) / 5 + 32
}

fn convert_to_celsius(temp: i32) -> i32 {
    (temp - 32) * 5/9
}
