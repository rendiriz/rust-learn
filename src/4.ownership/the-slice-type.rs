fn main() {
    // #4.3 The Slice Type
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("count word = {}", word);

    /*
      String Slices ============================================================
    */
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("slice hello = {}", hello);
    println!("slice world = {}", world);

    let s = String::from("hello");
    // both work
    // let slice = &s[0..2];
    let slice = &s[..2];
    println!("slice first = {}", slice);

    let s = String::from("hello");
    let len = s.len();
    // both work
    let slice = &s[3..len];
    // let slice = &s[3..];
    println!("slice last = {}", slice);

    let s = String::from("hello");
    // let len = s.len();
    // both work
    // let slice = &s[0..len];
    let slice = &s[..];
    println!("slice = {}", slice);

    let my_string = String::from("hello world");
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word_slice(&my_string[0..6]);
    // let word = first_word_slice(&my_string[..]);
    // // `first_word` also works on references to `String`s, which are equivalent
    // // to whole slices of `String`s
    // let word = first_word_slice(&my_string);
    println!("slice string = {}", word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    // let word = first_word_slice(&my_string_literal[0..6]);
    let word = first_word_slice(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    // let word = first_word_slice(my_string_literal);
    println!("slice string literal = {}", word);

    /*
      Other Slices =============================================================
    */
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    println!("slice array 0 = {}", slice[0]);
    println!("slice array 1 = {}", slice[1]);

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
