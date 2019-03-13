

fn main() {
    let mut a = 100;
    t(&mut a);
    println!("{}", a);
    println!("{}", u(&a));

    let b1 = String::from("left");
    let b2 = String::from("right");
    let c = b1 + &b2;

    println!("{}", c);

    panic!("this don't work");

}

fn t( v:  &mut i32) {
    *v += 1;
}

fn u(v: &i32) -> i32 {
    v + 1
}
