fn main() {
    let tup:(i32, i32, String) = (500, 6, "Hello".to_string());

    let (x, y, z) = tup;

    let a = tup.0;
    println!("The value of a is: {}", a);

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}