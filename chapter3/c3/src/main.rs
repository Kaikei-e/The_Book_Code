fn main() {
    println!("Hello World!!");

    let condition = false;
    let number = if condition {
        5
    }else {
        6
    };

    println!("The value of number is: {}", number)
}
