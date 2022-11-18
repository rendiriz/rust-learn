fn main() {
    // #2 Data Types

    // let guess: u32 = "not number".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {guess}");

    /*
      Integer Types
      i8 u8
      i16 u16
      i32 u32
      i64 u64
      i128 u128
      isize usizes
    */
    let integer: u32 = 100;
    println!("Integer: {integer}");

    /*
      Floating-Point Types
      f32
      f64
    */
    let float: f32 = 2.0;
    println!("Float: {float}");

    /*
      The Boolean Type
      bool
    */
    let f: bool = false;
    println!("Boolean: {f}");

    /*
      The Character Type
      char
    */
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Char: {z}");
    println!("Emotion: {heart_eyed_cat}");

    /*
      The Tuple Type
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let five_hundred = tup.0;
    println!("The value of x is: {five_hundred}");

    /*
      The Array Type
    */
    let a = [1, 2, 3, 4, 5];
    let element2 = a[2];
    let element4 = a[4];
    println!("The value of the element at index 2 is: {element2}");
    println!("The value of the element at index 4 is: {element4}");

    let a = [3; 5];
}
