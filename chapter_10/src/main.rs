use std::fmt::Debug;

//a generic function that cannot work without trait bounds

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn use_largest() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);

//     println!("The largest char is {}", result);
// }

/* ************************************************************************** */

#[derive(Debug)]
struct IntPoint {
    x: i32,
    y: i32,
}

pub trait Difference {
    fn difference(&self) -> i32;
}

impl Difference for IntPoint {
    fn difference(&self) -> i32 {
        self.x - self.y
    }
}

pub fn print_difference<T>(point: &T)
where
    T: Difference + Debug,
{
    println!("difference: {}", point.difference());
}
// ***** alternative syntaxes for trait bounds *****

// pub fn print_difference(point: &(impl Difference + Debug)) {
//     println!("difference: {}", point.difference());
// }

// pub fn print_difference<T: Difference + Debug>(point: &T) {
//     println!("difference: {}", point.difference());
// }

/* ************************************************************************** */

// **Challenge**: Implement a trait, then implement a generic type that satisifes
// the traits for different impl blocks depending on what the enclosed type is.

trait Bark {
    fn bark(&self);
}

struct Container<T> {
    value: T,
}

// The following two impl blocks solve the challenge

// impl Bark for Container<i32> {
//     fn bark(&self) {
//         println!("bark: {}", self.value);
//     }
// }

// impl Bark for Container<String> {
//     fn bark(&self) {
//         println!("bark: {}", self.value);
//     }
// }

// Conditionally impl a type for a generic type when the enclosed type
// implements a specific trait
// This is the better solution because it is more general
use core::fmt::Display;
impl<T: Display> Bark for Container<T> {
    fn bark(&self) {
        println!("bark: {}", self.value);
    }
}

fn use_bark() {
    let int_container = Container { value: 42 };
    let string_container = Container {
        value: "Hello".to_string(),
    };

    int_container.bark();
    string_container.bark();

    let char_container = Container { value: 'a' };
    char_container.bark();
}

/* ************************************************************************** */

fn some_borrow_practice() {
    // owned value travels with variable into outer scope
    let mut r = 3;
    {
        println!("r: {}", r);
        r = 2;
        println!("now r: {}", r);
    }
    println!("still r: {}", r);

    // borrowed value falls out of scope
    /*
    let y;
    {
        let x = 5;
        y = &x;
    }
    println!("y: {}", y);
    */

    // makes a reference to a new value by implicitly using _Copy_
    let y;
    {
        let x = &5;
        y = &*x;
    }
    println!("y: {}", y);
}

/* ************************************************************************** */

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn use_structs_containing_refereces() {
    // This does not work because the lifetime bounds on the struct are violated
    // The value referenced by the part field has a shorter lifetime than the struct
    /*
        let i;
        {
            let novel = String::from("Call me Ishmael. Some years ago...");
            let first_sentence = novel.split('.').next().expect("Could not find a '.'");
            i = ImportantExcerpt {
                part: first_sentence,
            };
        }
    println!("{:?}", i);
    */

    // This works because the value referenced by the part field has a lifetime
    // atleast as long as the struct

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);
}

fn main() {
    //A generic function that can't work without trait bounds
    //use_largest();

    //use a trait implemented for IntPoint
    let int_point = IntPoint { x: 1, y: 2 };
    int_point.difference();

    //use a function that requires trait bounds
    print_difference(&int_point);

    //conditionally implement traits based on enclosed types
    use_bark();

    some_borrow_practice();

    use_structs_containing_refereces();
}
