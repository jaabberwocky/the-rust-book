enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u8 {
    // borrow instead of taking ownership
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let c1 = Coin::Penny;
    let c2 = Coin::Nickel;
    let c3 = Coin::Dime;
    let c4 = Coin::Quarter;

    println!("c1: {}", value_in_cents(&c1));
    println!("c2: {}", value_in_cents(&c2));
    println!("c1 + c2 = {}", value_in_cents(&c1) + value_in_cents(&c2));
}
