use std::env::args;

pub fn even(x: &i32) -> bool {
    x % 2 == 0
}

fn main() {
    for x in args().skip(1) {
        println!("hello {}", x);
    }

    for x in 0..10 {
        println!("That's {}", x);
    }

    for (k, v) in (10..20).enumerate() {
        println!("k={},v={}", k, v);
    }

    for x in FibIter::new().take(15) {
        println!("fib={}", x);
    }

    let mut i = FibIter::new().filter(|x| x % 2 == 0).take(10);
    while let Some(v) = i.next() {
        println!("other fib={}", v);
    }
}

pub struct FibIter {
    a: i32,
    b: i32,
}

impl FibIter {
    pub fn new() -> Self {
        FibIter { a: 0, b: 1 }
    }
}

//Iterator is a trait more on those later
impl Iterator for FibIter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let c = self.a + self.b;
        self.a = self.b;
        self.b = c;
        Some(self.a)
    }
}
