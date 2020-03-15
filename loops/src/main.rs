// Loop types: loop, while, and for

fn main() {
    for_loop2();
}


fn loop_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {}", result);
}

// improve while_loop2
fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("value: {}", element);
    }
}

//improve while_loop
fn for_loop2() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!!");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!!");
}

// while_loop2
// 1. Could result in a panic if a.len() exceeded.
// 2. Could result in missed items if value < a.len().
// 3. Slower than for loop: checking condition each time.
fn while_loop2() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
