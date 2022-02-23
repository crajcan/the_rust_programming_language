use core::slice;
use std::rc::Rc;

// this will segfault usually
fn arbitrary_memory() {
    /*
        let address = 0x012345usize;
        let r = address as *const i32;

        unsafe {
            println!("r3 is: {}", *r);
        }
    */
}

fn rc_challenge() {
    let mut val = "string".to_string();

    let a = Rc::new(val);
    let b = Rc::clone(&a);

    // borrow after move
    // assert_eq!(val, "foobar".to_string());
    assert_eq!(*a, "string".to_string());
}

fn mess_with_addresses() {
    let pointer = &5;
    println!("address of pointer: {:?}", pointer);

    let address = 0x012345usize;
    let r = address as *const i32;

    // {:p} is the pointer formatter!
    println!("address of raw pointer: {:p}", r);
    println!("address of raw pointer as usize: {:?}", r as usize);
}

fn use_raw_pointers_to_break_ownership_rules() {
    let mut num = 5;

    let r1 = &num as *const i32;
    // we can create a mutable reference to a value that is already immutably referenced:
    let r2 = &mut num as *mut i32;
    // This could create a create a data race, if our code expected r1 not to change but
    // r2 was used to mutate the value.

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

unsafe fn more_dangerous() {
    println!("foobar");
}

// unsafe fns don't need to use unsafe to call another unsafe fn
unsafe fn dangerous() {
    more_dangerous()
}

fn use_an_unsafe_function() {
    //dangerous();

    unsafe {
        dangerous();
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); //creates a raw mutable pointer

    assert!(mid <= len);

    unsafe {
        // pointer addition to raw pointer to address of midpoint
        let midpoint = ptr.offset(mid as isize);

        (
            slice::from_raw_parts_mut(ptr, mid), //just creates a slice from a raw ptr and a len
            slice::from_raw_parts_mut(midpoint, len - mid),
        )
    }
}

fn unsafe_behind_interface() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // This seems unsafe
    a[0] = 8;
    println!("{}", r[0]);
    r[0] = 0;
    println!("{}", r[0]);
}

fn play_with_slices() {
    let v = vec![1, 2, 3, 4];
    //let r = &v;
    let slice = &v[..];

    let len = slice.len();

    //exclusive upper
    assert_eq!(&slice[..len], &[1, 2, 3, 4]);

    //inclusive lower
    assert_eq!(&slice[0..], [1, 2, 3, 4]);

    assert_eq!(&slice[len..], &[]);

    // Won't compile, indices must be usize;
    // let cant_slice = &slice[-1..];

    // panics, out of bounds
    // let inclusive_upper = &slice[..=len];

    // panics, out of bounds
    // let out_of_bound_slice = &slice[len + 1..];
}

fn play_with_mutable_slices() {
    let mut v = vec![0, 0];

    let first_mut_ref = &mut v[..];

    let (second_mut_ref, _third_mut_ref) = first_mut_ref.split_at_mut(1);

    second_mut_ref[0] = -1;
    println!("first_mut_ref[0]: \"{}\"", first_mut_ref[0]);
}

fn simpler_nll_example() {
    let mut v = vec![0, 0];

    let first_mut_ref = &mut v[..];
    let second_mut_ref = &mut first_mut_ref[..];

    // first_mut_ref[0] = -1;
    // println!("second_mut_ref[0]: \"{}\"", second_mut_ref[0]);

    second_mut_ref[0] = -1;

    // we can use first_mut_ref again because its borrows lifetimes have ended,
    // second_mut_ref's lifetime ended before the end of the scope due to NLL
    first_mut_ref[0] = 20;
    println!("first_mut_ref[0]: \"{}\"", first_mut_ref[0]);
}

fn compare_slices() {
    assert_eq!(vec![1, 2, 3, 4], [1, 2, 3, 4]);
    assert_eq!(vec![1, 2, 3, 4], &[1, 2, 3, 4]);

    // can't do this...why?
    // assert_eq!([1, 2, 3, 4], &[1, 2, 3, 4]);

    // Answer: deref on Vec<T> returns &[T], so deref coercion will call
    // *(vec![1,2,3,4].deref())
    // which becomes:
    // *(&[T])
    // *&[T]
    // [T]

    // Remember that _deref coercion_ only works when the passed in argument is a
    // reference toa type that implements `Deref` or a type that implements `Deref`. This is to avoid
    // implicitly creating references. So we can convert Vec<T> to &[T], or even [T], but it will not
    // convert &[T] to [T] because neither &[T] nor [T] implement Deref.
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn calling_c() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

static HELLO_WORLD: &str = "Hello, World!";

fn use_static() {
    println!("message is: {}", HELLO_WORLD);

    println!("address of HELLO_WORLD: {:p}", HELLO_WORLD);
    println!("address of HELLO_WORLD: {:p}", HELLO_WORLD);
}

fn make_const() -> &'static str {
    const foo: &str = "foobar";

    foo
}

fn use_const() {
    let c = make_const();
    println!("meessage is: {}", c);

    //foobar is supposed to change addresses or something but can't make it happen
    println!("address of foobar: {:p}", c);
    println!("address of foobar: {:p}", c);
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn mutate_static() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

struct Context<'a>(&'a str);

// Here we say that 2 distinct lifetimes are linked,
// 's lives at least as long as 'c, therefore:
// The string slice held by the Context the lives at least as long as the reference to it.
// struct Parser<'c, 's: 'c> {

// It's apparently enough now just to tell the compiler about the two different lifetimes:
struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(c: Context) -> Result<(), &str> {
    Parser { context: &c }.parse()
}

struct FooHolder<'a, T>(&'a T);

trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

fn use_trait_object_with_lifetimes() {
    let num = 5;

    let obj: Box<dyn Red> = Box::new(Ball { diameter: &num });
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn use_method_disambiguation() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;
struct Cat;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

impl Animal for Cat {
    fn baby_name() -> String {
        String::from("Kitten")
    }
}

fn use_fully_qualified_syntax() {
    println!("A baby cat is called a {}", Cat::baby_name()); //Kitten
    println!("A baby dog is called a {}", Dog::baby_name()); //Spot
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); //Puppy
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn use_supertraits() {
    let p = Point { x: 1, y: 3 };
    p.outline_print();
}

// Implemet OutlinePrint for Point<T> when T implements Display
// impl<T: fmt::Display> OutlinePrint for Point<T> {}

// impl Display for Point<T> whenever T implements Display
// impl<T: fmt::Display> fmt::Display for Point<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn use_newtype_wrapper() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

fn dynamically_sized_types() {
    //let s1: str = "hello world";
}

fn use_a_closure(f: impl Fn() -> ()) {
    f();
}

fn use_an_fnmut_closure(mut f: impl FnMut() -> ()) {
    f();
}

fn use_an_fnonce_closure(mut f: impl FnMut() -> ()) {
    f();
}

fn closure_traits_review() {
    let f = || println!("foobar");

    use_a_closure(f);
    use_an_fnmut_closure(f);
    use_an_fnonce_closure(f);
}

fn call_fn(f: fn() -> ()) {
    f();
}

fn call_closure(f: impl Fn() -> ()) {
    f();
}

fn hardcoded_fn() {
    println!("Used hardcoded fn");
}

fn create_closure() -> Box<dyn Fn() -> ()> {
    Box::new(|| println!("Used programmatically created closure!"))
}

fn use_function_pointers_or_closures() {
    call_fn(hardcoded_fn);
    call_fn(|| println!("Used hardcoded closure"));

    call_closure(create_closure());
    call_closure(|| println!("Used hardcoded closure"));
    call_closure(hardcoded_fn);

    // call_fn(create_closure());
}

trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.y,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(i32);
struct Meters(i32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

impl Point {
    fn x_value(&self) -> i32 {
        self.x
    }
}

fn use_x_value() {
    let p = Point { x: 42, y: 0 };

    println!("p.x_value(): {}", p.x_value());
    println!("Point::x_value(p): {}", Point::x_value(&p));
}

fn main() {
    arbitrary_memory();
    rc_challenge();
    mess_with_addresses();
    use_raw_pointers_to_break_ownership_rules();
    use_an_unsafe_function();
    unsafe_behind_interface();
    play_with_slices();
    play_with_mutable_slices();
    compare_slices();
    calling_c();
    simpler_nll_example();
    use_static();
    use_const();
    mutate_static();
    parse_context(Context("foobar"));
    use_trait_object_with_lifetimes();
    use_method_disambiguation();
    use_fully_qualified_syntax();
    use_supertraits();
    use_newtype_wrapper();
    closure_traits_review();
    use_function_pointers_or_closures();
    use_x_value();
}
