fn main() {
    // let x = 5;
    // let y = x + 3;
    // println!("x: {}\ny: {}", x, y);
   
    // another_function(5, "five");

    // let x = five();
    // println!("x is {}.", x);

    let lives = 14;
    println!("Lives: {}\nOne up!\nLives: {}", lives, one_up(14));
}

fn one_up(x: i32) -> i32 {
    x + 1
}

/*
fn five() -> u8 {
    // return 5;
    5
}

fn another_function(x: i32, y: &str) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}
*/

