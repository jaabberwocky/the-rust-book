use crate::garden::vegetables::Asparagus;

mod garden;

fn main() {
    let a = Asparagus {
        name: String::from("Johnny the Asparagus"),
    };

    println!("{:?}", a);
}
