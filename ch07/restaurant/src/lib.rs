mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// alter to not have to use the entire path
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // // relative path
    // front_of_house::hosting::add_to_waitlist();

    // use the path we defined above
    hosting::add_to_waitlist();
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        eat_at_restaurant();
    }
}
