fn main() {
    let numbers = input_two_number();
    println!("X = {}", numbers.0 + numbers.1);
}

fn input_two_number() -> (i32, i32) {
    let mut number1 = String::new();
    let mut number2 = String::new();

    std::io::stdin().read_line(&mut number1).expect("Failed to read line");
    std::io::stdin().read_line(&mut number2).expect("Failed to read line");

    let number1 = number1.trim().parse::<i32>().expect("Failed to parse number1");
    let number2 = number2.trim().parse::<i32>().expect("Failed to parse number2");

    return (number1, number2);
}