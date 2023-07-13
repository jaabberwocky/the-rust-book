fn main() {
    const MY_CONSTANT: u32 = 123456;

    let x = 5;
    let x = x + 7; // x is shadowed

    // x = x + 10; // error as you cannot re-assign to this variable

    let mut y = 10;
    y = y + 15;

    println!("MY_CONSTANT: {}\nx: {}", MY_CONSTANT, x);
    println!("y: {}", y);
}
