/*pub enum Result<T, E> {
    Ok(T),
    Err(E),
}*/

use std::env::args;
use std::fs::File;
use std::io;
use std::io::Read;

#[derive(Debug)]
pub enum MyErr {
    IOErr(std::io::Error),
}

// if there were many Error types in being joined into this one type then each From means you can
// just use into and all the errors work together
// You can also use match on teh error type to decide on your next action
impl From<io::Error> for MyErr {
    fn from(e: io::Error) -> Self {
        MyErr::IOErr(e)
    }
}

fn main() {
    for a in args().skip(1) {
        let w = count_ws(&a);
        let w2 = count_ws2(&a);

        println!("{}, has {:?} 'w's", a, w);
        if let (Ok(a), Ok(b)) = (&w, &w2) {
            assert_eq!(a, b);
            println!("All OK");
        }
        //let g = w2.expect("Shold we have an error here");
        //println!("g = {}", g);

        //you dont have to use match to make small changes  map_err also helps to change the error
        //type
        let big = w.map(|n| n > 10);
        println!("big = {:?}", big);
    }
    println!("Hello, world!");
}

pub fn count_ws(filename: &str) -> Result<i32, MyErr> {
    let mut f = match File::open(filename) {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };
    let mut s = String::new();
    if let Err(e) = f.read_to_string(&mut s) {
        return Err(MyErr::IOErr(e));
    }
    let mut res = 0;
    for c in s.chars() {
        //chars reades unicode points one at a time
        if c == 'w' {
            res += 1;
        }
    }

    Ok(res)
}

pub fn count_ws2(filename: &str) -> Result<i32, MyErr> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s.chars().fold(0, |n, c| if c == 'w' { n + 1 } else { n }))
}
