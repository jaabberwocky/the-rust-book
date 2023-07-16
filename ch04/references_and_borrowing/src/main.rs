#[derive(Debug, Clone)] // used to get "easy" print statements
struct SimpleStruct {
    num: i32,
}

fn main() {
    let ss = SimpleStruct { num: 1 };

    let mut ss1 = ss;
    // println!("{:?}", ss); // this won't compile as ownership is transferred to ss1 and ss ceases to exist
    // but this would work
    let mut ss2 = &mut ss1; // we move from ss1 to ss2
    println!("{:?}", ss2);

    add_to_ss(&mut ss2, 30); // we mutate ss2 variable
    println!("{:?}", ss2);

    // cannot have more than one reference (mutable or read-only) if there
    // is already a mutable reference
    let mut ss3 = SimpleStruct { num: 5 };
    {
        // ss4 as a mutable ref ENDS at the end of this scope block
        let ss4 = &mut ss3;
    }
    let ss5 = &ss3; // will not compile

    println!("{:?}", ss5);
}

fn add_to_ss(ss: &mut SimpleStruct, val: i32) {
    ss.num += val;
}
