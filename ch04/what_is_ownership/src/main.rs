fn main() {
    // string literals are immutable
    let s: &str = "hello";

    println!("s: {}", s);
    // let s = "world"; // error: cannot assign twice to immutable variable `s`

    // use a String type which allows for mutation
    let mut h = String::from("hello world");

    println!("h: {}", h);
    h.push_str(" my friend!");
    println!("h: {}", h);
}
