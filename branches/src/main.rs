fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    let div = 3;
    if number % div == 0 {
        println!("{} is divisible by {}", number, div);
    } else {
        println!("{} is not divisible by {}", number, div);
    }
}
