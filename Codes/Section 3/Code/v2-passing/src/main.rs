fn main() {
    let mut x = 7;
    let mut y = 15;
    let pt = choose(&mut x, &mut y);

    *pt += 2;
    //x cannot be even read here because pt is borring it in a mutable way.
    //one mutable borrow or many immutable borrows are allowed.
    println!("pt = {}", pt);
    x += 3;
    println!("x = {},y = {}", x, y);

    //Pointers in structs
    //

    let th = make_thing(&x, y);

    y += 2;
    //x += 2;
    println!("Thing = {:?}", th);
    println!("x ={}, y={}", x, y);
}

pub fn choose<'a>(x: &'a mut i32, y: &'a mut i32) -> &'a mut i32 {
    if rand::random() {
        x
    } else {
        y
    }
}

pub fn make_thing<'a>(a: &'a i32, b: i32) -> SomeThing<'a> {
    SomeThing { pt: a, age: b }
}

#[derive(Debug)]
pub struct SomeThing<'a> {
    pt: &'a i32,
    age: i32,
}
