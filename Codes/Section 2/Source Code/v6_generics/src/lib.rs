use std::ops::{Add, AddAssign};

#[derive(PartialEq, Debug, Clone)]
pub struct Point<N> {
    x: N,
    y: N,
}

impl<N> Point<N> {
    pub fn new(x: N, y: N) -> Self {
        Point { x, y }
    }
}

impl<N: AddAssign + Clone> Point<N> {
    pub fn transpose(&mut self, p: &Self) {
        self.x += p.x.clone();
        self.y += p.y.clone();
    }
}

//lets us use the + operator
impl<N: Add<Output = N> + Clone> Add for Point<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let p = Point::new(1, 7);
        let mut p2 = Point::new(2, 3);
        let p3 = p + p2.clone();
        assert_eq!(p3, Point::new(3, 10), "Points dont match");

        p2.transpose(&Point::new(6, 6));

        let ps = Point::new("hello", "goodbye");
        //assert_eq!(p3, ps);
    }
}
