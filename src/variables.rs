fn main() {
    // #1 Variables and Mutability

    /*
      Mutability
      let mut
    */
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /*
      Constants
      const
    */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of constant is: {THREE_HOURS_IN_SECONDS}");

    /*
      Shadowing
    */
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
