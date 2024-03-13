//Box is pointing where the value store in memory

//      Drop Trait allows you to customize what happen when a value goes out of scope
//cant call drop() method manually (c.drop) //use drop fn (drop(c))
//don't worry about cleaning up resources and memory   RUST WILL DO IT AUTOMATICALLY
// _!_  Clean up happen automatically when value goes out the scope

//Reference counting smart Pointer is only usefully in Single Threaded Program

//Interior Mutability check value at runtime

use std::borrow::Cow;
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::{Rc, Weak};
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    // Cons(i32, Box<List>),     //Cons(i32, List), // in recursive enum
    // Cons(i32, Rc<List>), //use Reference pointer
    //Cons(Rc<RefCell<i32>>, Rc<List>), //going to be mutable
    Cons(i32, RefCell<Rc<List>>), //for reference cycles
    Nil,
}
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    child: RefCell<Vec<Rc<Node>>>,
}

//............................................................................
struct MyBox<T>(T);
impl<T> MyBox<T> { fn new(x: T) -> MyBox<T> { MyBox(x) } }

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 } }
//............................................................................
fn main() {
   // let list = Cons(1, Cons(2, Cons(3, Nil)));
   // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
   // let list_1 = Cons(5, Box::new(list)); //list has been moved into list_1
   // let list_2 = Cons(25, Box::new(list));

//............................................................................
//     //construct instances of the reference counting smart poiter
//     //provide multiple owners
//     let list = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("Count after creating list = {}", Rc::strong_count(&list));
//
//     let _list_1 = Cons(3, Rc::clone(&list));     // just increases a reference count
//     //doesn't make deep copies of data like most clone impl calling
//     println!("Count after creating list_1 = {}", Rc::strong_count(&list));
//     {
//         let _list_2 = Cons(4, Rc::clone(&list));
//         println!("Count after creating list_2 = {}", Rc::strong_count(&list));
//     }
//     println!("Count after list_1 goes out of scope = {}", Rc::strong_count(&list));
//......................................................................................
//
//     let value = Rc::new(RefCell::new(5));
//
//     let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
//     let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
//     let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
//
//     //dereferences value*
//     *value.borrow_mut() += 10;
//     println!("a after = {:?}", a);
//     println!("b after = {:?}", b);
//     println!("c after = {:?}", c);
//......................................................................................

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a init rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b  init rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("d rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    //that we have a cycle
    //it will overflow the stack
    //println!("a next item = {:?}", a.tail()); //logical bug
    //......................................................................
            let leaf = Rc::new(Node {
                value: 3,
                parent: RefCell::new(Weak::new()),
                child: RefCell::new(vec![]),
            });
            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
            println!("leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf), //ownership count
                Rc::weak_count(&leaf) //dont have ownership count
            );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            child: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {}, weak = {}",
                 Rc::strong_count(&branch), //ownership count
                 Rc::weak_count(&branch) //dont have ownership count
        );
        println!("leaf strong = {}, weak = {}",
                 Rc::strong_count(&leaf), //ownership count
                 Rc::weak_count(&leaf) //dont have ownership count
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}",
             Rc::strong_count(&leaf), //ownership count
             Rc::weak_count(&leaf) //dont have ownership count
    );
    //......................................................................

//......................................................................................
    let x = 5;
    //let y = Box::new(x);    //Box is pointing where the value store in memory
    let y = MyBox::new(x);
    assert_eq!(5, x); //assert_eq!(5, *y);
    assert_eq!(5, *(y.deref())); // * deref trait

    let m = MyBox::new(String::from("Sergi"));
    hello(&m); //&MyBox<String> -> &String -> &str //or// hello(&(*m)[..]);

}

fn hello(name: &str) {
    println!("Hello, {name}");
}