fn main() {
    // #4.2 References and Borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    /*
      Mutable References =======================================================
    */
    // let s = String::from("hello"); // This error use mutation
    let mut s = String::from("hello");

    change(&mut s);
    println!("s = {}", s);

    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2); // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    /*
      Dangling References ======================================================
    */
    // let reference_to_nothing = dangle(); // This error use directly
    let reference_to_nothing = no_dangle();
    println!("reference_to_nothing = {}", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// This error use directly
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
