#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{cell::RefCell, ops::Deref, rc::Rc};

mod alternate_mutable_list;
mod mutable_list;
mod tree;

fn practice_with_box_ownership() {
    let x = "Foobar".to_string();
    let y = &x;

    assert_eq!("Foobar".to_string(), x);
    assert_eq!("Foobar".to_string(), *y);

    takes_ownership(x);
    // borrow after move
    // println!("y: {}", y);

    let a = "Foobar".to_string();
    let b = Box::new(a);

    // borrow after move
    // We can't use a after putting it in a box because boxes own their content
    // Notice that while b owns its content while y does not.
    //assert_eq!("Foobar".to_string(), a);

    assert_eq!("Foobar".to_string(), *b);

    // Here we move ownership of the box b itself
    takes_ownership(b);
    // borrow after move
    // println!("*b: {}", *b);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

/*
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}
*/

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn our_own_smart_pointer() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);

    assert_eq!(5, *y);
}

fn deref_coercion() {
    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let x = MyBox::new(String::from("rust"));
    hello(&x);
}

fn deref_coercion_visualized() {
    let _a = &MyBox(String::from("Rust"));
    let _b = *(&(MyBox(String::from("Rust")).deref())); // calls `deref` on MyBox<String>
    let _c = *(&(&String::from("Rust"))); // `deref` turns MyBox into &String
    let _d = &String::from("Rust"); // & and * cancel
    let _e = "Rust"; // calls `deref` on &String, to get &str
}

fn canceling_references_and_dereferences() {
    let x = "Foobar".to_string();
    assert_eq!("Foobar".to_string(), *&x);
    assert_eq!("Foobar".to_string(), &*x);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with value: {}!", self.data);
    }
}

fn using_the_drop_trait() {
    let c = CustomSmartPointer {
        data: "Foobar".to_string(),
    };
    let d = CustomSmartPointer {
        data: "BizBuz".to_string(),
    };

    println!("CustomSmartPointers created.");

    // explicit destructor calls not allowed
    // c.drop();

    // reverses the order of the drops
    drop(c);

    println!("end of method");
}

enum BoxList {
    Cons(i32, Box<BoxList>),
    Nil,
}

fn using_box_for_multiple_owners() {
    let a = BoxList::Cons(5, Box::new(BoxList::Cons(10, Box::new(BoxList::Nil))));
    let b = BoxList::Cons(3, Box::new(a));
    // borrow after move
    // let c = BoxList::Cons(4, BoxList::new(a));
}

enum ReferenceList<'z> {
    Cons(i32, &'z ReferenceList<'z>),
    Nil,
}

fn using_reference_list_for_multiple_references() {
    let a = ReferenceList::Cons(5, &ReferenceList::Cons(10, &ReferenceList::Nil));
    let b = ReferenceList::Cons(3, &a);
    let c = ReferenceList::Cons(4, &a);
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn using_reference_counter_for_multiple_owners() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

fn play_with_reference_counts() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }

    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn trying_to_break_rc() {
    let a = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));

    {
        let alist = Rc::new(a);
        let b = Cons(3, Rc::clone(&alist));
        let c = Cons(4, Rc::clone(&alist));
    }

    // It won't let us use the reference to a anymore because we didn't bind
    // the variable a to Rc<List>.
    // borrow after move
    // println!("a: {:?}", a);
}

fn usecase_for_refcell() {
    let mut m = "foo".to_string();
    let n = &m;
    // cannot assign to *n, which is behind a `&` reference
    // *n = "baz".to_string();

    let mut x = "foo".to_string();
    let y = Box::new(x);

    // cannot assign to *y as y is not declared as mutable.
    // *y = "baz".to_string();

    let mut a = "foo".to_string();
    let b = RefCell::new(a);

    *b.borrow_mut() = "baz".to_string();

    println!("b: {:?}", b);
}

fn main() {
    practice_with_box_ownership();

    our_own_smart_pointer();

    deref_coercion();

    deref_coercion_visualized();

    canceling_references_and_dereferences();

    using_the_drop_trait();

    using_box_for_multiple_owners();

    using_reference_list_for_multiple_references();

    using_reference_counter_for_multiple_owners();

    play_with_reference_counts();

    mutable_list::use_mutable_list();

    alternate_mutable_list::use_alternate_mutable_list();

    tree::use_tree();

    tree::use_tree_and_print_reference_counts();

    trying_to_break_rc();

    usecase_for_refcell();
}

fn takes_ownership<T>(_x: T) {}
