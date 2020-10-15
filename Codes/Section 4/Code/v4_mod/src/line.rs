use v6_generics_do_not_use::Point;

pub fn dist(a: &Point<f64>, b: &Point<f64>) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}
