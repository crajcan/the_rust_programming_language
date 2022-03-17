#![allow(dead_code)]

fn use_arithemtic_remainder_and_assignment() {
    let mut x = 4;
    x %= 2;
    println!("x: {}", x)
}

fn use_arithmetic_negation() {
    let n = 4;
    println!("n: {}", n);
    println!("-n: {}", -n);
}

struct Car {
    model: String,
    bed_length: Option<u32>,
}

struct Foo {
    x: u32,
    y: u32,
}

fn use_and_the_rest() {
    let tup = (500, 6.4, 1);
    let (x, ..) = tup;

    println!("x coordinate: {}", x);

    let foo = Foo { x: 1, y: 3 };
    let Foo { x, .. } = foo;

    println!("x coordinate: {}", x);

    let car = Car {
        model: "Mustang".to_string(),
        bed_length: Some(5),
    };
    let Car { model, .. } = car;

    println!("model: {}", x);
}

fn use_raw_string_literals() {
    let x = r"foo";
    let y = r#"f"o"#;
    let z = r##"f#o"##;
}

fn use_turbofish() {
    let matrix = <Vec<Vec<i32>>>::new();
    let other_matrix = <Vec<Vec<i32>>>::new();
    assert_eq!(matrix, other_matrix);
}

fn use_single_element_tuple_expression() {
    let x = (4,);

    println!("{:?}", x);
}

fn use_single_element_tuple_type() {
    let x: (i32,) = (4,);
    println!("{:?}", x);
}

fn main() {
    use_arithemtic_remainder_and_assignment();
    use_arithmetic_negation();
    use_and_the_rest();
    use_raw_string_literals();
    use_turbofish();
    use_single_element_tuple_expression();
    use_single_element_tuple_type();

    let x: [i32; 5] = [4; 5];

    assert_eq!(arr[..], [1, 2, 3, 4, 5]);
    assert_eq!(arr[..3], [1, 2, 3]);
}
