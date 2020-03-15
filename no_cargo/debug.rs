/// Documentation should be generated from this
/// since there are three slashes instead of two.

fn main() {
    #[derive(Debug)]
    struct Person<'a> {
        // The 'a defines a lifetime
        name: &'a str,
        age: u8,
    }

    //let name = "Mike";
    //let age = 31;
    //let mike = Person {name, age};
    let mike = Person {name: "Mike", age: 31};

    // {} for reg variables
    // {:?} for ...
    // add hash to pretty print: {:#?}
    println!("{:#?}", mike);
}

