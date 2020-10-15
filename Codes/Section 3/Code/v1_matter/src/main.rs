fn main() {
    //the compiler checks against the function signature not the contents of the function
    let y = get_y();
    println!("y is {}", y);

    let x = 4;
    //int implements Copy this is ok as it does not contain any pointers
    mydrop(x);
    println!("x = {}", x);

    let s = "hello".to_string();
    mydrop(s);
    println!("s = {}", s);
    //s was moved into the function "drop"
    //we did not send a pointer.
    //s contains a pointer, so if the drop function drops it,
    //we can't use it after that.
}

pub fn get_y<'a>() -> &'a i32 {
    let mut y = 13;
    let n = &mut y;
    *n += 3;
    //Local variables are destroyed when the function ends
    //You cannot point to them afterwards
    &y
}

pub fn mydrop<T>(_: T) {}
