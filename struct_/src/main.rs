#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn create(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rec = Rectangle {
        width: 12,
        height: 34,
    };
    let rec1 = Rectangle {
        width: 22,
        height: 50,
    };
    let rec3 = Rectangle::create(25);

    println!("rec: {:#?}", rec);
    println!("Area rec: {}", rec.area());
    println!("rec can hold rec1: {}", rec.can_hold(&rec1));

    println!("{:?}", rec3);
}