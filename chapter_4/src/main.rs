fn main() {
    let mut s = String::from("hello");
    let world = " world!";
    s.push_str(world); // appends a literal to a string by cloning
    println!("{}", s);

    let x = 5;
    let y = x; //copy
    println!("x: {}", x);
    println!("y: {}", y);

    let s1 = String::from("hello");
    let _s2 = s1; // move
                  //borrow after move
                  //println!("s1: {}", s1);

    /*
    error[E0382]: borrow of moved value: `s1`
    --> src/main.rs:13:24
    |
    11 |     let s1 = String::from("hello");
    |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    12 |     let s2 = s1;
    |              -- value moved here
    13 |     println!("s1: {}", s1);
    |                        ^^ value borrowed here after move
    */

    let s1 = "hello";
    let s2 = s1; //copy
    println!("s1: {}", s1);
    println!("s2: {}", s2);

    // We can't do this because s1 could be of any size, and by derereferencing
    // we would be trying to put an arbitrary amount of data onto the stack
    //let bads = *s1;

    let my_string = String::from("hello world");
    let my_string_literal = "hello world";

    // works on a slice of a String
    let _word = first_word(&my_string[..]);

    // works on a slice of a string literal
    let _word = first_word(&my_string_literal[..]);

    //works on a string literal directly
    let word = first_word(my_string_literal);

    assert_eq!(word, "hello");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}
