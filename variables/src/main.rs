fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    let word = "Hello";
    println!("{}", word.len());

    // Can change type with let
    let spaces = "    ";
    println!("Spaces: '{}'", spaces);
    let spaces = spaces.len();
    println!("Spaces: '{}'", spaces);

    /* Cannot change type w/o let, even w/mut
     * let mut space = "    ";
     * spaces = spaces.len();
     */

    let dec = 98_222;
    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1111_0000;
    let byt = b'A';
    let byt_arr = b"Hi!";

    println!("{}", dec);
    println!("{}", hex);
    println!("{}", oct);
    println!("{}", bin);
    println!("{}", byt);
    println!("{:?}", byt_arr);

    let tup = (31, "Mike", 'H');
    let (x, y, z) = tup;
    println!("y: {}", y);

    let tup2: (char, u8, &str) = ('M', 23, "Bulls");
    println!("Index 2: {}", tup2.2);

    let arr: [char; 4]  = ['a', 'b', 'c', 'd'];
    let arr_len = arr.len();
    println!("array (len: {}) is: {:?}", arr_len, arr);

    let rep_arr = [4; 6];
    println!("{:?}", rep_arr);

    let num_arr = [3, 5, 8, 3, 2];
    println!("Index 2: {}", num_arr[2]);
}

