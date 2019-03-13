

fn main() {
    let mut a = 100;
    t(&mut a);
    println!("{}", a);
    println!("{}", u(&a));
}

fn t( v:  &mut i32) {
    *v = *v + 1;
}

fn u(v: &i32) -> i32 {
    v + 1
}
