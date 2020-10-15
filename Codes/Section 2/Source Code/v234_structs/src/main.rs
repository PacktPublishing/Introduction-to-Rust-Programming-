// i8 i16 i32 i64 isize
// u8 u16 u32 u64 usize
// bool
// str --- more later

#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
pub struct Ship {
    location: Point,
    status: ShipStatus,
}

#[derive(Debug, PartialEq)]
pub enum ShipStatus {
    Engaged,
    Waiting,
    Firing(Point),
    Heading(Point),
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Shot { from: Point, at: Point },
    Move { from: Point, to: Point },
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn transpose(&mut self, p: &Point) {
        self.x += p.x;
        self.y += p.y;
    }
}

impl Ship {
    pub fn attack(&mut self, p: Point) {
        self.status = ShipStatus::Firing(p)
    }

    pub fn time_step(&mut self, v: &mut Vec<Action>) {
        use std::cmp::Ordering::*; //an enum
        match &self.status {
            ShipStatus::Heading(p) => {
                let nx = match p.x.cmp(&self.location.x) {
                    Greater => self.location.x + 1,
                    Less => self.location.x - 1,
                    Equal => self.location.x,
                };
                let ny = match p.y.cmp(&self.location.y) {
                    Greater => self.location.y + 1,
                    Less => self.location.y - 1,
                    Equal => self.location.y,
                };
                v.push(Action::Move {
                    from: self.location.clone(),
                    to: Point::new(nx, ny),
                });
            }
            ShipStatus::Firing(p) => v.push(Action::Shot {
                from: self.location.clone(),
                at: p.clone(),
            }),
            _ => {}
        }
    }
}

fn main() {
    let mut ship = Ship {
        location: Point { x: 10, y: 10 },
        status: ShipStatus::Waiting,
    };
    ship.location.x += 10;
    println!("ship location = ({},{})", ship.location.x, ship.location.y);
    println!("ship = {:?}", ship);

    let mut a = Point::new(10, 4);
    let b = Point::new(20, -4);
    //a becomes self
    a.transpose(&b);
    // panics if not equal and ends the program immediately
    assert_eq!(a, Point::new(30, 0));

    ship.attack(b);
    println!("Attacking Ship = {:?}", ship);

    let mut actions: Vec<Action> = Vec::new();
    ship.time_step(&mut actions);
    println!("actions = {:?}", actions);

    if let ShipStatus::Firing(p) = ship.status {
        println!("the ship will fire at {:?}", p);
    }
}
