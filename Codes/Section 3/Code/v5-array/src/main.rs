fn main() {
    // Arrays are fixed sized lists.
    // The size is part of the type.
    let a: [i32; 3] = [4, 5, 6];
    println!("a = {:?}", a);
    let mut b = [9, 8, 7, 6];
    println!("b = {:?}", b);
    //b = a; fixed size
    let c = [0; 5];
    println!("c = {:?}", c);
    println!("");

    //Vecs are resizable

    let mut v = Vec::new();
    v.push(3);
    v.push(4);
    println!("v = {:?}, len = {}, cap = {}", v, v.len(), v.capacity());
    v.extend(a.iter());
    v.extend(b.iter());
    println!("v = {:?}, len = {}, cap = {}", v, v.len(), v.capacity());

    println!("");

    //Slices are views into Vecs or arrays

    let sa = &a[..2];
    println!("sa = {:?}", sa);

    let sb = &mut b[2..];
    sb[1] = 12;
    println!("modified b = {:?}", b);

    let v2 = vec![3, 5, 3, 6, 2];
    println!("v2={:?}", v2);

    let vs = &v2[1..4];
    println!("vs={:?}", vs);
}
