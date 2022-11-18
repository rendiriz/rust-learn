fn main() {
    // #3 Functions

    println!("Hello, world!");
    another_function();

    /*
      Parameters
    */
    another_function_parameters(5);

    /*
      Statements and Expressions
    */
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    /*
      Functions with Return Values
    */
    let z = another_function_return();
    println!("The value of z is: {z}");

    let plus = plus_one(5);
    println!("The value of plus is: {plus}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_parameters(x: i32) {
    println!("The value of x is: {x}");
}

fn another_function_return() -> i32 {
    100
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
