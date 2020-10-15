fn main() {
    let s: &'static str = "  中文 please  ";
    println!("s = '{}'    pt={},len={}", s, s.as_ptr() as usize, s.len());
    let s2 = s.trim();

    println!(
        "s2 = '{}'    pt={},len={}",
        s2,
        s2.as_ptr() as usize,
        s2.len()
    );

    //A different string, it has been copied here.
    //String can be appended to but str cannot
    let mut ss = String::from(s);
    ss.push('&');
    ss.push_str(" sandwiches");

    //This is a reference to the copy
    let ss2: &str = &ss;
    println!(
        "ss2 = '{}'    pt={},len={}",
        ss2,
        ss2.as_ptr() as usize,
        ss2.len()
    );
}
