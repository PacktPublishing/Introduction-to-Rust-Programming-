use std::ops::Add;

pub struct Doubler<I: Iterator> {
    inner: I,
}

impl<I: Iterator<Item = O> + Sized, O: Clone + Add> Iterator for Doubler<I> {
    type Item = O::Output; // Add has an Ouput type
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|a| a.clone() + a)
    }
}

pub trait Double: Iterator + Sized {
    fn double(self) -> Doubler<Self> {
        Doubler { inner: self }
    }
}

//Here's the magic
impl<I: Iterator<Item = O> + Sized, O: Add + Clone> Double for I {
    //Here we could override the double method if we wanted, but it already does what we want.
}

fn main() {
    for x in (0..10).double() {
        println!("doubled ={}", x);
    }
    println!("Done");
}
