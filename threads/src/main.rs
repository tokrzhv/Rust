use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};


fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handels = vec![];

    for _ in 1..11 {
        let counter = Arc::clone(&counter);
        let handel = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handels.push(handel);
    }

    for handel in handels {
        handel.join().unwrap();
    }
    println!("Result : {}", *counter.lock().unwrap());

//......................................................................................
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("{:?}", m);
//......................................................................................
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        // let msg = String::from("hi");
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        //tx.send(msg).unwrap(); //send take ownership
        for value in values {
            tx.send(value).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    thread::spawn(move || {
        let values = vec![
            String::from("hi2"),
            String::from("from2"),
            String::from("the2"),
            String::from("thread2"),
        ];
        for value in values {
            tx2.send(value).unwrap();
            thread::sleep(Duration::from_millis(3000))
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

//..............................................................................................................
//     let handle = thread::spawn(|| {      //then main thread ends -> spawn thread stop execution use ->__join
//         for i in 1..11 {
//             println!("hi number {} from the spawned threads!", i);
//             thread::sleep(Duration::from_millis(1))
//         }
//     });
//     handle.join().unwrap();
//     for i in 1..5 {
//         println!("hi number {} from tha main thread!", i);
//         thread::sleep(Duration::from_millis(1))
//     }
//     //handle.join().unwrap();
//
// //.............................................................
//     let v = vec![1, 2, 3, 4, 5];
//     let handle_1 = thread::spawn(move || {
//         println!("here is a vector: {:?}", v);
//     });
//     handle_1.join().unwrap();


}
