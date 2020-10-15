use std::fmt::Debug;
use std::fmt::Display;

trait TimeTrait: Debug + Display {
    fn time(&mut self, _t: f64) {}
}

//impl<T: Debug + Display> TimeTrait for T {}

#[derive(Debug)]
pub struct Train {
    speed: f64,
    dist: f64,
}

impl Display for Train {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Train is at:{},going at {}", self.dist, self.speed)
    }
}

impl TimeTrait for Train {
    fn time(&mut self, t: f64) {
        self.dist += self.speed * t;
    }
}

#[derive(Debug)]
pub struct Car {
    speed: f64,
    ang: f64,
    dx: f64,
    dy: f64,
}

impl Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Car is at:{},{},going at {}",
            self.dx, self.dy, self.speed
        )
    }
}

impl TimeTrait for Car {
    fn time(&mut self, t: f64) {
        self.dx += self.speed * t * self.ang.sin();
        self.dy -= self.speed * t * self.ang.cos();
    }
}

fn main() {
    let mut v: Vec<Box<dyn TimeTrait>> = Vec::new();
    v.push(Box::new(Car {
        speed: 2.,
        dx: 0.,
        dy: 0.,
        ang: std::f64::consts::PI / 2.,
    }));
    v.push(Box::new(Train {
        speed: 2.,
        dist: 5.,
    }));

    println!("v = {:?}", v);
    for t in &mut v {
        t.time(2.);
    }
    println!("v = {:?}", v);
}
