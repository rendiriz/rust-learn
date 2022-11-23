fn main() {
    // #4.1 What Is Ownership?

    // === Ownership Rules =====================================================
    /*
      The String Type
    */
    // let s = String::from("hello"); // This error
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    /*
      Ways Variables and Data Interact: Move
    */
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // This error because moved
    println!("{}, world!", s2);

    /*
      Ways Variables and Data Interact: Clone
    */
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    /*
      Stack-Only Data: Copy
      Here are some of the types that implement Copy:
      - All the integer types, such as u32.
      - The Boolean type, bool, with values true and false.
      - All the floating point types, such as f64.
      - The character type, char.
      - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
    */
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // === Ownership and Functions =============================================
    let s = String::from("hello");
    let t = s.clone();
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    // println!("s = {}", s); // This error because ownership use clone or reference
    println!("t = {}", t); // This work use clone
    println!("x = {}", x);

    // === Return Values and Scope =============================================
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s1 = {}", s1);
    // println!("s2 = {}", s2); // This error because ownership use clone or reference
    println!("s3 = {}", s3);

    /* Using Tuple */
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
