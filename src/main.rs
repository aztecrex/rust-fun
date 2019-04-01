



fn main() {

    let a = (1, 2, 3, 4);

    match &a {
        (first, .., 7) => println!("Hey {}", first),
        (.., mid, _) => println!("Hey {}", mid),
    }


}
