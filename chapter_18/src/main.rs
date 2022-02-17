#![allow(unused_variables)]
#![allow(dead_code)]

fn use_while_let() {
    let mut stack = vec![];

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }
fn use_for_loop_enumerate() {
    let v = vec!['a', 'b', 'c'];

    if let Some((i, val)) = v.iter().enumerate().next() {
        println!("{} is my index, {} is my value", i, val);
    }

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("current location: ({}, {})", x, y);
}

// ************* Match Ergonomics RFC Demonstration *******************
// Notice the following signature doesn't work because `String` is not Copy, so this would try to
// acquire ownership from a reference:
//
// fn print_strings2(&(x,y): &(String, String)) {

// To fix this we can remove the leading `&` to create
// x and y as references via destructuring with match ergonomics:
//
// fn print_strings((x, y): &(String, String)) {
//
// A more detailed explanation can be found here:
// https://stackoverflow.com/questions/56511328/how-does-rust-pattern-matching-determine-if-the-bound-variable-will-be-a-referen\
// But what it expands to is this:
fn print_strings2(&(ref x, ref y): &(String, String)) {
    println!("The two strings: ({}, {})", x, y);
}

fn match_in_closure_parms() {
    let f = |&(x, y)| {
        println!("current location: ({}, {})", x, y);
    };

    f(&(3, 5));

    // Doesn't work with non-Copy types
    // let g = |&(x, y)| {
    //     println!("Some Strings: ({}, {})", x, y);
    // };

    // g(&("Foo".to_string(), "Fuz".to_string()));

    // We could leverage match ergonomics again, just saying:
    let h = |(x, y): &(String, String)| {
        println!("some Strings: ({}, {})", x, y);
    };

    h(&("Foo".to_string(), "Fuz".to_string()));
}

fn ranges() {
    let es = 1..5;

    for e in es {
        println!("r[{}]: {}", e - 1, e);
    }

    let is = 1..=5;

    println!("inclusives");
    for i in is {
        println!("i[{}]: {}", i - 1, i);
    }
}

#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn print_coordinates_again(p: &Point) {
    // match ergonomics again
    let Point { x, y } = p;
    println!("I'm at {}, {}!", x, y);
}

fn which_axis(p: &Point) {
    match p {
        Point { x: 0, y: 0 } => println!("On both axis at ({},{})", p.x, p.y),
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x: _, .. } => println!("On neither axis!"),
    }
}

fn destructuring_structs() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    print_coordinates_again(&p);
    which_axis(&p);
}

struct Holder {
    val: String,
}

// Why are we allowed to use the pattern `|&Point { x, y }| in the argument
// definition for the closure, but we get a 'cannot move out shared context'
// error when we destructure with "Some(&p) =>..."?
//
// Answer: The difference here is that in the closure, we are specifying that
// the argument should be a reference to a `Point`, and then we are copying
//  x & y. In the match expression, we are trying to transfer ownership of the
// value pointed to by the &Point from the array to the variable 'p'.
//
// Notice when we try to use the same pattern from some_of_squares in
// "sum_of_strings" we get the same "cannot move out shared reference" error
fn destructuring_references() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    // remove the '&' to see "match ergonomics" in action
    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
    println!("sum of squares: {}", sum_of_squares);

    // match points.iter().next() {
    //     Some(&p) => println!("got a point: ({},{})", p.x, p.y),
    //     None => println!("no more points!"),
    // }

    let _strings = vec![
        Holder {
            val: "foo".to_string(),
        },
        Holder {
            val: "baz".to_string(),
        },
    ];

    // let sum_of_strings: usize = _strings.iter().map(|&Holder { val }| val.len()).sum();
    // println!("sum of strings: {}", sum_of_strings);
}

// thought this was ok
//
// Answer: You're essentially trying to move the value pointed to by x into y.
// But x doesn't own the value so you can't transfer ownership from it.
fn challenge_string() {
    let x = &"foo".to_string();

    //let &y = x;
    //assert_eq!(y, "foo".to_string());
}

// thought this was ok too
//
// You can't bind a str to a local var because it doesn't have a size known at
// compile time. It is unsized, as is [T]
// fn challenge_str() {
//     let x = "foo"; //     let &y = x;

//     assert_eq!(y, *"foo");
// }

#[derive(Debug)]
struct StringHolder {
    val: Option<String>,
}

fn borrow_nested_data_that_is_owned() {
    let holder = StringHolder {
        val: Some("Owned robot name".to_string()),
    };

    // Here we could remove the `ref` and use a "partial move" to move the string
    // into a new variable. We use `ref` instead to create an immutable reference
    // and let `holder` retain ownership.
    match holder {
        StringHolder { val: Some(ref str) } => {
            println!("Borrowed owned data via destructuring: {}", str);
        }
        _ => (),
    }

    // here we again use as_ref() to avoid a partial move
    let holder_str = holder.val.as_ref().unwrap();
    println!("Borrowed owned data via as_ref: {}", holder_str);

    println!("holder retains ownership: {:?}", holder);
}

fn borrow_nested_data_that_is_borrowed() {
    let holder = StringHolder {
        val: Some("Borrowed robot_name".to_string()),
    };
    let holder_ref = &holder;

    // Here we are basically copying an immutable reference.
    // The `ref` is no longer necessary. Since we're destructuring a reference,
    // match ergonomics kicks in
    match holder_ref {
        StringHolder {
            val: Some(/*ref*/ str),
        } => {
            println!("Borrowed borrowed data via destructuring: {}", str)
        }
        _ => (),
    }

    // Here we need as_ref() because unwrap takes ownership by default,
    // and we can't take ownership from a reference.
    // pub const fn unwrap(self) -> T {
    let holder_str = holder_ref.val.as_ref().unwrap();
    println!("Borrowed borrowed data via #as_ref: {}", holder_str);
}

fn mutably_borrow_nested_data_that_is_owned() {
    let mut holder = StringHolder {
        val: Some("Owned robot name".to_string()),
    };

    match holder {
        StringHolder {
            val: Some(ref mut str),
        } => {
            *str = "foobar".to_string();
        }
        _ => (),
    }

    println!("mutably borrow data via ref mut: {:?}", holder);

    let holder_handle = holder.val.as_mut().unwrap();
    *holder_handle = holder_handle.to_owned() + "fiz_buz";

    println!("mutably borrow data via #as_mut: {:?}", holder);
}

fn mutably_borrow_nested_data_that_is_mutably_borrowed() {
    let mut holder = StringHolder {
        val: Some("Mutably borrowed robot_name".to_string()),
    };
    let holder_ref = &mut holder;

    // Here we are basically copying an immutable reference.
    // The `ref mut` is no longer necessary. Since we're destructuring a reference,
    // match ergonomics kicks in
    match holder_ref {
        StringHolder {
            val: Some(/*ref mut*/ str),
        } => {
            *str = "foobar".to_string();
        }
        _ => (),
    }
    println!(
        "mutably borrow from a mutable borrow with destructuring: {:?}",
        holder
    );

    let holder_handle = holder.val.as_mut().unwrap();
    // apparently this call to #to_owned() doesn't break the ownership rules
    // because by the time the expression is, done evaluating, holder_handle
    //  has a new value.
    *holder_handle = holder_handle.to_owned() + "fiz_buz";

    println!(
        "mutably borrow from a mutable borrow with as_mut: {:?}",
        holder
    );
}

// `#as_str` requires ownership of the `String`, so we use `#take` to take ownership of the
// `String` and temporarily replace it with `None` so we can call `#as_str` on `x` before
// we mutate val. Note this doesn't work with #unwrap, which attempts to transfer ownership
// of the same value
fn practice_using_take_again() {
    let robot_name = &mut StringHolder {
        val: Some("Borg".to_string()),
    };

    // 1st try won't work can't transfer ownership:
    //robot_name.val = Some(robot_name.val.unwrap().as_str().to_string() + " blick");

    // Here we use `#take` in chain:
    robot_name.val = Some(robot_name.val.take().unwrap().as_str().to_string() + " Blick");

    // #as_ref is probably better:
    robot_name.val = Some(robot_name.val.as_ref().unwrap().as_str().to_string() + " Blick");

    // take in destructuring
    match robot_name.val.take() {
        Some(x) => robot_name.val = Some(x.as_str().to_string() + " Blick"),
        None => (),
    }

    // Borg Blick Blick Blick
    println!("robot_name: {:?}", robot_name);
}

fn match_guards() {
    let num: Option<i32> = Some(6);

    match num {
        Some(5) | Some(6) if num.unwrap().is_positive() || num.unwrap() % 2 == 0 => {
            println!("positive or even 5 or 6 found")
        }
        Some(x) => println!("x: {}", x),
        None => (),
    }

    //let foo = Some("foo".to_string());

    // We cannot transfer ownership (move) it a pattern guard:
    // match foo {
    //     Some(x) if x + "bar" == "foobar".to_string() => println!("got part of foobar"),
    //     Some(x) => println!("string is not part of foobar"),
    //     None => println!("got nothing"),
    // }

    //let foo = &mut Some("foo".to_string());

    // We also cannot borrow as mutable in a pattern guard
    // match foo {
    //     Some(x) if x.as_bytes_mut() == [b'f', b'o', b'o', b'b', b'a'] => println!("got fooba"),
    //     Some(x) => println!("string is not fooba"),
    //     None => println!("got nothing"),
    // }
}

fn at_bindings() {
    struct User {
        id: i32,
    }

    let msg = User { id: 5 };

    match msg {
        User { id: my_id @ 3..=7 } => println!("Found an id in range: {}", my_id),
        User { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        User { id } => println!("Found some other id: {}", id),
    }
}

fn main() {
    use_while_let();
    use_for_loop_enumerate();
    print_coordinates(&(3, 5));
    //print_strings(&("foo".to_string(), "bar".to_string()));
    print_strings2(&("foo".to_string(), "bar".to_string()));
    match_in_closure_parms();
    ranges();
    destructuring_structs();
    destructuring_references();
    challenge_string();
    //challenge_str();
    borrow_nested_data_that_is_owned();
    borrow_nested_data_that_is_borrowed();
    mutably_borrow_nested_data_that_is_owned();
    mutably_borrow_nested_data_that_is_mutably_borrowed();
    practice_using_take_again();
    match_guards();
    at_bindings();
}

fn takes_ownership<T>(x: T) {}

fn takes_ownership_and_returns_something<T>(x: T) -> String {
    "Nom".to_string()
}
