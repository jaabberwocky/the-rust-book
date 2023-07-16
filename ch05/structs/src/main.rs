#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut u1 = User {
        active: true,
        username: String::from("James"),
        email: String::from("james.tobin@gmail.com"),
        sign_in_count: 4,
    };

    println!("u1: {:?}", u1);

    // access specific information
    println!("u1.username: {}", u1.username);

    // changing speciifc information
    u1.sign_in_count += 1;
    println!("u1.sign_in_count: {}", u1.sign_in_count);

    // struct update syntax
    let u2 = User {
        email: String::from("hello@test-accounts.com"),
        ..u1
    };

    println!("u2: {:?}", u2);
    println!("u1: {:?}", u1); // note that compiler error here as parts of u1 is moved to u2
}
