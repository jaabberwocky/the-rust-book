enum IpAddrKind {
    V4(String),
    V6(String),
}

// store different types of data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// we can implement methods on enums
impl Message {
    fn call(&self) {
        if let Message::Write(s) = self {
            println!("Writing message: {}", s);
        } else {
            println!("Call method on Message invoked");
        }
    }
}

// store structs directly
enum ChessPiece {
    King(Piece),
    Queen(Piece),
}

struct Piece {
    // define Piece struct values
}

fn main() {
    // we get "free" constructors with the enum variant taking
    // in data to be stored
    let v4 = IpAddrKind::V4(String::from("127.0.0.1"));
    let v6 = IpAddrKind::V4(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let m2 = Message::Move { x: 1, y: 2 };
    m2.call();

    // showcasing option values
    //
    let some_number = Some(5);
    let some_char = Some('a');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // expect 10, but won't compile
    // let sum = x + y;

    // handle both scenarios of both Some and None
    match y {
        Some(i) => println!("Sum of {} + {}: {}", x, i, x + i),
        None => println!("No value for y"),
    }
}
