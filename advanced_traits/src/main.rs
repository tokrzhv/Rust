use std::ops::Add;
use std::fmt;

struct Millim(u32);
struct Meters(u32);
impl Add<Meters> for Millim {
    type Output = Millim;

    fn add(self, other: Meters) -> Millim {
        Millim(self.0 + (other.0 * 1000))
    }
}
//........................................................................................
trait Pilot {
    fn fly();
}
trait Wizard {
    fn fly();
}
struct Human;
impl Human {
    fn fly() {
        println!("human");
    }
}
impl Pilot for Human {
    fn fly() {
        println!("Pilot");
    }
}
impl Wizard for Human {
    fn fly() {
        println!("from wizard");
    }
}
//........................................................................................
pub trait Iterator<T> {
    //Associated types
   // type Item; // has 1 type for implementation
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator<u32> for Counter {
   // type Item = u32;

    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}
impl Iterator<u16> for Counter {
    //type Item = u16;

    fn next(&mut self) -> Option<u16> {
        Some(0)
    }
}
//.......................................................................................

struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn  fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

struct Age(u32);
struct ID(u32);

//.......................................................................................
fn main() {

    let w = Wrapper(
        vec![String::from("Hello"), String::from("World")]
    );
    println!("w = {w}");


//.......................................................................................


//.......................................................................................
   // let person = Human; // Pilot::fly(&person); //person.fly();
    <Human as Wizard>::fly();

    println!("Hello, world!");
}