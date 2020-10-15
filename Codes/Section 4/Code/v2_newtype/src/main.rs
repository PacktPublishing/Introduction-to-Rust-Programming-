use std::ops::Add;

#[derive(Debug)]
pub struct AVec<T>(Vec<T>);

impl<T> Add for AVec<T> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self.0.extend(rhs.0.into_iter());
        self
    }
}

fn main() {
    let v = AVec(vec![1, 5, 6]);
    let v2 = AVec(vec![2, 6, 7]);
    let v3 = v + v2;

    println!("v3 = {:?}", v3);
}
