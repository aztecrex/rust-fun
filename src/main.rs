


#[derive(Debug, Clone)]
struct Q {
    x: i32,
}


fn main() {

    let a = Q{ x:100 };
    // {
        let mut b = a;
        b.x += 100;
        println!("{:?}", b);
    // }
    println!("{:?}", a);

}
