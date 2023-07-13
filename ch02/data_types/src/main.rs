fn main() {
    // scalar types
    // 4 of them

    // integer types
    let x1: i8 = 45;
    let x2: u8 = 123;
    let x3: i16 = 45;
    let x4: u16 = 123;

    println!("x1: {}\nx2: {}\nx3: {}\nx4: {}", x1, x2, x3, x4);

    // fp types
    let x5: f32 = 3.14159;
    let x6: f64 = 3.141598;

    // char type
    let x7: char = 'A';

    // compound types
    // 2 of them

    // array type
    // useful when you want to have data allocated on the stack rather than heap
    let x8: [i32; 5] = [1, 2, 3, 4, 5];

    // tuple type
    let x9: (i32, f64, char) = (1, 3.14159, 'A');

    // destructuring to get the tuple values
    let (y1, y2, y3) = x9;

    println!("x9 (only one of the values): {}", x9.2);
    println!("y1: {}\ny2: {}\ny3: {}", y1, y2, y3);
}
