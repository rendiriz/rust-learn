// like interface
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {
    // println!("Hello, world!");

    // #5.1 Defining and Instantiating Structs
    // let user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // mutable
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("email = {}", user1.email);

    // function
    let em = String::from("third@example.com");
    let un = String::from("third");

    let build_user = create_user(em, un);
    println!("email = {}", build_user.email);
    println!("username = {}", build_user.username);

    // Creating Instances From Other Instances With Struct Update Syntax
    let user2 = User {
        email: String::from("fourty@example.com"),
        ..user1
    };

    println!("email = {}", user2.email);

    // Using Tuple Structs without Named Fields to Create Different Types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Structs Without Any Fields
    let subject = AlwaysEqual;
}

fn create_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        // sorthand
        // email,
        // username,
        active: true,
        sign_in_count: 1,
    }
}
