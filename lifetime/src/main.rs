//generic lifetime annotation
// create relationships between lifetime of mult ref

//  1.  Each parameter that is a ref gets its own lifetime parameter
//  2.  If there is exactly one input lifetime parameter,
//      that lifetime is assigned to all output lifetime parameters
//  3.  If there are mut lifetime parameters, but one of them &self or &mut self
//      the lifetime of SELF is assigned to all output lifetime parameters

use std::fmt::Display;

struct ImpEcpert<'a> {
    part: &'a str,
}

impl<'a> ImpEcpert<'a> {
    fn return_part(&'a self, announcement: &str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
//............................................................................

fn longest_with_an_announsment<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }

}
fn main() {
    let novel = String::from("Call me.Some years ago..");
    let first_sentence = novel.split('.').next().expect("Couldn't find..");
    let i = ImpEcpert {
        part: first_sentence,
    };

//...............................................
    let str1 = String::from("qwertyuiop");
    let str2 = String::from("qwerty");
    let ann = String::from("Strings comparing");
    let result = longest_with_an_announsment(str1.as_str(), str2.as_str(), ann);
    println!("The longest string is: {result}");
}


//  $i32            // a ref
//  &'a i32         // a ref with an explicit lifetime
//  &'a mut i32     // a mut ref with an explicit(явний) lifetime

// fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     }else {
//         y
//     }
// }
