#[derive(Debug)]
pub enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

pub fn use_mutable_list() {
    println!("\nUsing a Mutable List \n");

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // here the Rc is automatically dereferenced, then borrow_mut returns a
    // RefMut<T> and finally we use the dereference operation to change the
    // inner value of the RefMut<T>

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let (c_value, c_cons) = match c {
        Cons(ref v, ref l) => (v, l),
        Nil => panic!("Expected Cons, got Nil"),
    };

    *c_value.borrow_mut() += 10;

    println!("after mutating c, c: {:?}", c);

    println!("c_cons: {:?}", c_cons);
    println!("     a: {:?}", a);

    // Note that we can't modify the "next" Rc<List> in the Cons for a, b, or c
    // because interior mutability is only supported for the first value in the
    // Cons tuple because it is wrapped in a RefCell
}
