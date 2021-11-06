fn main() {
    let y = None;

    if let Some(3) = y {
        println!("found 3!");
    } else {
        println!("No, this is a y");
    }

    let x = Some(3);

    if let Some(3) = x {
        println!("found 3!");
    }
    // You can't use if let to test types
    //  let y = "Foobar";

    // if let Some(3) = y {
    //         println!("found 3!");
    //     }
    // let z = Some("Foobar");
    //
    //     if let Some(3) = z {
    //         println!("found 3!");
    //     }
    let mut a = Some(Rectangle {
        width: 30,
        height: 50,
    });

    double_if_exists(&mut a);

    println!("mut a is now: {:?}", a);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
note: you can't change a string slice at all, it's not owned!
will have to use an owned String and try to modify it.
*/
fn double_if_exists(b: &mut Option<Rectangle>) {
    match b {
        Some(c) => {
            double(c);
        }
        None => (),
    }
}

fn double(d: &mut Rectangle) {
    d.width *= 2;
    d.height *= 2;
}
