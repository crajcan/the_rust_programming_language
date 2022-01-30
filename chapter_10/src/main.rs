use std::fmt::Debug;

struct Point<T> {
    x: T,
    y: T,
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

#[derive(Debug)]
struct IntPoint {
    x: i32,
    y: i32,
}

/******************************************** */

pub trait Difference {
    fn difference(&self) -> i32;
}

impl Difference for IntPoint {
    fn difference(&self) -> i32 {
        self.x - self.y
    }
}

// pub fn print_difference(point: &(impl Difference + Debug)) {
//     println!("difference: {}", point.difference());
// }

// pub fn print_difference<T: Difference + Debug>(point: &T) {
//     println!("difference: {}", point.difference());
// }

pub fn print_difference<T>(point: &T)
where
    T: Difference + Debug,
{
    println!("difference: {}", point.difference());
}

// **Challenge**: Implement a trait, then implement a generic type that satisifes the traits for
// different impl blocks depending on what the enclosed type is.

/******************************************** */
trait Bark {
    fn bark(&self);
}

struct Container<T> {
    value: T,
}

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

use core::fmt::Display;
impl<T: Display> Bark for Container<T> {
    fn bark(&self) {
        println!("bark: {}", self.value);
    }
}

/******************************************** */
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    /******************************************** */
    let int_point = IntPoint { x: 1, y: 2 };
    print_difference(&int_point);

    /******************************************** */

    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);

    // println!("The largest char is {}", result);

    /******************************************** */
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    /******************************************** */
    let int_container = Container { value: 42 };
    let string_container = Container {
        value: "Hello".to_string(),
    };

    int_container.bark();
    string_container.bark();

    let char_container = Container { value: 'a' };
    char_container.bark();
    /******************************************** */

    let mut r = 3;
    {
        println!("r: {}", r);
        r = 2;
        println!("now r: {}", r);
    }
    println!("still r: {}", r);

    /*
    let y;
    {
        let x = 5;
        y = &x;
    }
    println!("y: {}", y);
    */

    // making a new reference with copy?
    let y;
    {
        let x = &5;
        y = &*x;
    }
    println!("y: {}", y);

    /******************************************** */
    /*
        let i;
        {
            let novel = String::from("Call me Ishmael. Some years ago...");
            let first_sentence = novel.split('.').next().expect("Could not find a '.'");
            i = ImportantExcerpt {
                part: first_sentence,
            };
        }
    */

    println!("{:?}", i);
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{:?}", i);
}
