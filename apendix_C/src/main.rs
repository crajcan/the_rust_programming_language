#![allow(unused_imports)]

use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Debug, Default)]
struct Foo {
    x: u32,
    y: u32,
}

#[derive(PartialEq, PartialOrd, Debug)]
enum Shape {
    Rectangle(u32, u32),
    Circle(u32),
}

fn use_partial_eq() {
    let a = Foo { x: 1, y: 1 };
    let b = Foo { x: 1, y: 1 };

    if a == b {
        println!("foobar!")
    }

    assert_eq!(a, b);

    let x = Shape::Rectangle(1, 2);
    let y = Shape::Rectangle(1, 2);
    let z = Shape::Rectangle(2, 2);

    assert_eq!(x, y);
    assert_ne!(x, z);
}

fn use_partial_ord() {
    let r = Shape::Rectangle(1, 2);
    let c = Shape::Circle(2);

    assert!(r < c);

    let a = Foo { x: 1, y: 1 };
    let b = Foo { x: 1, y: 2 };

    assert!(a < b);

    let c = Foo { x: 2, y: 1 };
    let d = Foo { x: 1, y: 2 };

    assert!(c > d);
}

fn use_default() {
    let foo = Foo {
        x: 1,
        ..Default::default()
    };

    println!("foo: {:?}", foo);
}

fn main() {
    use_partial_eq();
    use_partial_ord();
    use_default();
}
