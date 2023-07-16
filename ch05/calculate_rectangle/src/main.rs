#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// make struct methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn get_fmt_str(&self) -> String {
        return format!("Rectangle: {}, {}", self.width, self.height);
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // all functions in impl block are associated functions
    // associated functions are functions that don't take self as a parameter
    // String::from is one of those
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let r = Rectangle {
        width: 30,
        height: 50,
    };

    let r2 = Rectangle {
        width: 10,
        height: 20,
    };

    let sq1 = Rectangle::square(10);

    assert_eq!(r.can_hold(&r2), true);
    assert_eq!(r.area(), 1500);
    println!("r >>> {}", r.get_fmt_str());
    println!("r2 >>> {}", r2.get_fmt_str());
    println!("sq1 >>> {}", sq1.get_fmt_str());
}
