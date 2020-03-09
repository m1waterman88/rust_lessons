// Formatted printing
// https://doc.rust-lang.org/stable/rust-by-example/hello/print.html


fn main() {
    let age = 31;
    println!("I'm {} years old.", age);

    println!("I was {1}, but now I'm {0}.", age, (age - 1));

    println!("The {subject} {verb} the {object}.",
             object="the lazy dog",
             subject="quick brown fox",
             verb="jumps over");

    // {:b} changes the value to binary: output of 2 is 10.
    println!("{} of {:b} people know binary; the other half doesn't.", 1, 2);

    // Right-align text with a specified width. This will output: "     1"
    // 5 white spaces and a "1."
    println!("{number:>width$}", number=1, width=6);

    // Or pad with zeros/whatever you want
    println!("{number:>0width$}", number=1, width=6);
    println!("{number:>>width$}", number=1, width=6);

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
}

