fn main() {
    let y = get_y();
    println!("Hello, {}", y);

    let t = BinTree {
        data: 3,
        left: Some(Box::new(BinTree {
            data: 5,
            left: None,
            right: None,
        })),
        right: None,
    };

    println!("t = {:?}", t);
}

pub fn get_y() -> Box<i32> {
    //Box is a pointer to the heap
    //when the box is dropped, it so does what it's referencing.
    //We call them owned pointers.
    let x = 32;
    Box::new(x)
}

#[derive(Debug)]
pub struct BinTree<T> {
    data: T,
    //pointers have a fixed size, so we know how big the struct will be
    left: Option<Box<BinTree<T>>>,
    right: Option<Box<BinTree<T>>>,
}
