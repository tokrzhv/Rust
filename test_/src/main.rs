use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::fs::{self, File};
use std::io;
use std::io::Read;
use colored::*;

//.......................................................................................................
fn read_from_file() -> Result<String, io::Error>{
    let mut str = String::new();
    File::open("somefile.txt")?.read_to_string(&mut str)?;
    Ok(str)
    // fs::read_to_string("somefile.txt")  // witch returns a result (string with file_data or error) //use std::fs::{self};
}
//.......................................................................................................

fn main() {
    //const CON: i32 = 100_000;
//...........................................................................................


//...........................................................................................
    //fn some_fn<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
    // fn some_fn<T, U>(t: &T, u: &U) -> i32
    // where   T: Display + Clone,
    //         U: Clone + Debug
    // {
    //     //
    // }
//...........................................................................................
    println!("Data from file: {:?}", read_from_file().unwrap());
//.......................................................................................................
    let text = "This is is some rand rand is text..";
    let mut words_count = HashMap::new();
    for word in text.split_whitespace()  {
        let count = words_count.entry(word).or_insert(0);
        *count += 1;//return mut ref for value
    }
    for (key, value) in &words_count {
        println!("{}: {}", key.bright_magenta(), format!("{}", value).italic().bright_yellow());
    }   // let _ = &words_count.iter().for_each(|(key, value)| {});
//.......................................................................................................


}

//....................Scalar
//Int
//Float
//Bool
//Char
//......................Compound
//tuple
//arrays
//vectors

//........................................................................................
//...Ownership
// 1. Each value has a variable that's called its owner
// 2. There can only be ine owner at a time.
// 3. When the owner goes out of scope, the value will be dropped
//  { s is not valid here, don't declared
//      let s = String::from("some string"); // accessible
//  } s is no longer valid
//...........................................................................................

//...References
// 1. At any given time, you can have either one mut ref
// or any number of immutable ref
// 2. Ref must always be valid
//...........................................................................................
