union IntOrFloat {
    i: u32,
    f: f32,
}

fn main() {
    let u = IntOrFloat { f: 11.0 };

    unsafe {
        match u {
            IntOrFloat { f: 10.0 } => println!("Found ten!"),
            IntOrFloat { i } => println!("Found an int: {}", i),
        }
    }
}
