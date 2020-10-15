//Can mutate internals without needing mut
use std::cell::RefCell;
//Manages Reference Counting
//A pointer and a pointer to a counter, both owned
//when cloned it ups the count by one.
use std::rc::Rc;

//Atomic Reference Count, does the counting in uninteruptable atomic actions
//This means it can be shared across multiple threads
use std::sync::Arc;

//Mutable interior, but with atomic locking
use std::sync::Mutex;

fn main() {
    let counter = Rc::new(RefCell::new(0));
    let c2 = counter.clone();
    for _ in 0..10 {
        let mut m = counter.borrow_mut();
        *m += 1;
    }

    for _ in 0..10 {
        let mut m = c2.borrow_mut();
        //let _mc = counter.borrow(); // already borrowed panic
        *m += 1;
    }

    println!("counter = {:?}", counter);

    let tsafe = Arc::new(Mutex::new(0));
    let t2 = tsafe.clone();
    let handle = std::thread::spawn(move || {
        for x in 0..10 {
            let mut m = t2.lock().unwrap();
            *m += 1;
            println!("{} : T2 modding, x = {}", *m, x);
            drop(m);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    for x in 0..10 {
        let mut m = tsafe.lock().unwrap();
        *m += 1;
        println!("{} : T1 modding, x = {}", *m, x);
        drop(m);
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    handle.join().unwrap();
}
