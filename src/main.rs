


#[derive(Debug)]
struct P {x: String, y: i32 }

fn main() {

    let p = P {x: "fun".to_string(), y: 33};
    match &p {
        P {x, y: 0} => {
            println!("x is {}, y is 0", x);
        },
        P {x, y} => println!("{} {}", x, y),
    }
    println! ("{:?}",p); // OK, match borrowed it

    match p {
        P {x, y: 0} => {
            println!("x is {}, y is 0", x);
        },
        P {x, y} => println!("{} {}", x, y),
    }
    // println! ("{:?}",p); // NOT OK, match moved it

}
