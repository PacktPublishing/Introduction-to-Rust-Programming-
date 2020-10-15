mod hello;
mod line;
use v6_generics_do_not_use::Point;

fn main() {
    hello::hw();
    let p1 = Point::new(1.,1.);
    let p2 = Point::new(4.,5.);
    println!("dist = {}",line::dist(&p1,&p2));
    local::do_thing();
}

mod local {
    pub fn do_thing() {
        println!("thing done");
    }
}
