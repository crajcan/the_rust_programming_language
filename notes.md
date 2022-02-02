# Notes From The Text

These are notes that I took every time I encountered a fact I didn't already know, or was introduced to
a concept that I didn't immediately understand

## Chapter 1

```
run clippy: cargo clippy --fix --allow-staged || --allow-dirty
run rustfmt: rustfmt src/main.rs || src.lib.rs
```

## Chapter 2

`let mut guess = String::new();`

Creates a string, which is a growable UTF-8 encoded piece of text

The `::` indicates this is an 'assocatiated method', implemented on a type, not a instance of that type. Sometimes referred to as a static method.

```
use std::io; == std::io::stdin() 
io::stdin()
```

`io::stdin()`

creates an instance of `std::io::Stdin()`, which is a type that represnets a handle to the standard input for the terminal.

`io::stdin().read_line(&mut guess)`

The `&` just indicates that the argument is a reference. Just like variables, references are immutable by default. Thus the need for the `&mut`. 

```
.expect("Failed to read line");
```

`expect` is implemented specifically for std::io::Result and takes in a std::io::Result. It panics if that Result is an instance of Err, or unpacks return value if the Result is an instance of Ok.

`cargo update` will ignore versions pinned in `Cargo.lock` and update to the dependency to the latest version that fits the api specification of the version pinned in Cargo.toml. It will not make major version updates.

`extern crate rand;` - Lets the compiler know we'll be using rand as an externa dependency.

`use rand::Rng;` - brings the Rng trait into scope, which defines random number generation methods on instances of `rand`.

## Chapter 3

Integers can be written with a type suffix instead of a type annotation:

```
21u8
4123i32
```

Integer literals can be defined in several bases:

```
Base           Example
Decimal        98_222
Hex            0xff
Octal          0o77
Binary         0b1111_0000
Bytes(u8 only) b'A'
```

Using pattern matching to destructure tuples:

```
let tup = (500, 6.4, 1);
let (x,y,z) = tup;
```

Indexing into a tuple:

```
let five_hundred = tup.0
```

### Arrays
- Must have a fixed length, they cannot grow or shrink once declared.
- Can only hold values of one type.
- allocated on the stack by default
- Trying to access an index out of bounds results in a panic

```
let a = [1,2,3,4,5];
```

indexing into an array:

```
println!("first val: {}", a[0]);
```

## Chapter 4

- for each _value_ in rust, there is one _variable_ that is its _owner_.
- When the _owner_ goes out of scope, the _value_ is dropped.

A string literal hardcoded into the program, immutable:

```
let s = "hello";
```

### String
- growable and heap allocated. The location, length, and capacity are stored on the stack.

Create from a string literal:

```
let s = String::from("hello");
```

`drop` is called when s goes out of scope. `drop` is the function where the implmentor of `String` can put the code to return the memory to the operating system. 

```
let x = 5;
let y = x;
```    
for values held on the stack, the second variable gets a copy of the value

#### Move
```
let x = String::from("hello");
let y = x;
```    
When we assign y to x, the String data from the stack (location, length, capacity) are copied for y, but the head data (the actual string) remains the same and is not copied. This is a 'moved' value. When the data is moved into y, x is considered invalid, and doesn't need to be freed.

For heap allocated data, Rust will always make a move like this instead of a deep copy. To make a deep copy, you have to explicitly call "clone".

For stack types (like integers) we can implment copy, to avoid moves. This will only work if drop has not been implemented for the type or any of it's components. why?

#### Copy types

1. integers
2. bool
3. chars
4. floats
5. tuples containing only the above

#### Ownership with functions
passing a variable to a function will copy or move just as assigning to a new variable does.

```
let s1 = String::from("hello");
takes_ownership(s1);
```
Here, s1 is moved into takes ownership, and cannot be used again.

#### Borrowing & References
We can pass a _reference_ so that we can avoid the _move_ while still letting the called function _borrow_ the data it needs.

```
let s1 = String::from("hello");
let len = calculate_length(&s1);
println!("The length of '{}' is {}.", s1, len);

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Here, calculate length borrows s1, so we can still us it in the caller.

The `&` in `calculate_length(s: &String)` denotes that `s` is a _reference_. 

The `&` in `let len = calculate_length(&s);` is used to create a reference to the _String_ `s1`.

Since `s1` was already a string, technically, `s` points to `s1`, which in turn points to heap allocated data.

- Note that calculate_length does not need to dereference s in order to use it.

#### Mutable References

```
fn main() {
    let mut s = String::from("hello");
    
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

To allow a called function to modify borrowed data, we have to do three things.

1. Tell the compliler that the data we are allocating is going to be mutable with a type annotation.

```
let mut s = String::from("hello");
```

2. Denote that the function parameter on the called function is a mutable reference. 

```
fn change(some_string: &mut String) {
```

3. Create a mutable reference to the data to pass as an argument to the called function.

```
change(&mut s);
```

- You can only have one mutable reference at a time, which prevents _data races_.

A _data race_ occurs when:

1. Two or more pointers access the same data at the same time.
2. At least one of the pointers is being used to write to the data.
3. There's no mechanism being used to synchronize access to the data.

Similarily, when you have a mutable reference, you cannot have any immutable references.

#### Danginling references

Rust won't let you have a reference to something that has been freed (dropped)

```
fn dangle() -> &String {
    let s = String::From("hello");

    &s
}
```
This won't compile because `s` has gone out of scope

Instead, just return the value;
```
fn dangle() -> &String {
    let s = String::From("hello");

    s
}
```

#### Recap of Reference Rules:

- You may have 1 mutable reference _or_ any number of immutable references.
- Referecnes must always be valid.

### String Slice
- A reference to part of a String
- Anotated as `&str`
- Does not have ownership. Because a String slice is an immutable borrow, we cannot have a mutable borrow to the String it points to while it is in scope. 

```
let s = String::from("hello world");

let hello = &s[..5];
let world = &s[6..];
```
`hello` and `world` are stored on the stack as a ptr into the middle of the String `s` on the heap and a length value that stores the length of the slice.

- String literals also ahve the type &str, because they are slices that point into the binary. 
- We prefer to define parameters as `first_word(s: &str)` instead of `first_word(s: &String)` because it allows us to use the same function on String and &str values.

fn main() {
    let my_string = String::from("hello world");
    let my_string_literal = "hello world"; 

    // works on a slice of a String
    let word = first_word(&my_string[..]);

    // works on a slice of a string literal
    let word = first_word(&my_string_literal[..]); 
      
    //works on a string literal directly
    let word = first_word(my_string_literal);
}

- In the eyes of the compiler, both a `String Slice` and an borrowed `String` are `&str`

## Chapter 5

### Structs

In order for a struct to store a reference type, such as `&str` (the string slice type), we must use a lifetime specifier to ensure that the data referenced by a struct is valid for as long as the struct is. Without lifetime specifiers, structs can only store owned types like `String`.

methods: on an instance of a struct.
"assocatiated functions": on a struct (type), similar to "class methods" or "static methods".

#### Printing Structs

We cannot use `{}` without manually implmenting the Disply trait for a given struct. We can use `{:?}` if we include the `#[derive(debug)]` annotation immediately above the struct definition.

`#[derive(debug)]` not only gives us use of `{:?}` but also `{:#?}`, which will print the same thing but with new lines between the fields of the struct.

#### Automatic Referencing and Dereferencing

When calling methods, Rust can automatically dereference a pointer create a reference from a piece of data to match the signature of the method you are trying to call.

For instance if you try to call `p1.distance(&p2)`, and the signtature of `distance` is `distance(&self, p2: &Point)`, the complier will behind the scenes create a reference out of p1 so that you can call `#distance` without explicitly creating the ptr yourself:

```
p1.distance(&p2) --> (&p1).distance(&p2)
```

In addition to adding a `&` to automatically create a reference, rust can automatically create a mutable reference by adding `&mut` if the given method signature calls for it (`foo(&mut self)`)

Conversely, when you provide a pointer to an instance and try to call a method on the instance, Rust can automatically dereference a ptr by adding a `*` behind the scenes.

#### Timing of creating references and mutable references

- You can create a mutable ref after you already have an immutable ref, as long as you don't use that same immutable ref again later.
- you can create an immutable ref after you created a mutable ref, as long as you don't try to use that same mutable ref again later.

## Chapter 6 (Enums and Pattern Matching)

Chapter 6 was pretty straight forward. As a challenge:

Write a method that takes in a mutable reference, then based on a match statement, conditionally passes the mutable reference to another method that mutates it. (Completed in #double_if_exists)

## Chapter 7 (Module stuff I mostly knew)

## Chapter 8 (Array and collection stuff I knew)

## Chapter 9 (Panics and Error Handling)

#### match vs ?

Difference between using match to handle Result type and using `?` operator to handle result type:

When you use `?` the `from` function from the `From` trait is used to convert the error to the type specified in the function signature.

`unwrap` and `expect` are signs that your program can be more robust (handle failure instead of panicking)

#### Wrapping types for consiseness:

It's sometimes useful to wrap a simple value, like a u32 from the guessing game, in a type:

```
pub struct Guess {
    value: u32,
 }
```

And then define validations on the type in the `#new` associated function in the impl block for that type, instead of repeating the runtime validations (value is between 0 and 100) everywhere the value is used.

#### Definition of Result:

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
****
Notice that T and E are unbounded.

## Chapter 10 (Generics, Traits, and Lifetimes)

Using many generic type parameters indicates that your code needs restructuring into smaller pieces (types).

**Challenge**: Implement a trait, then implement a generic type that satisifes the traits for different impl blocks depending on what the enclosed type is (Solved with Bark & Container in *chapter_10/src/main.rs*)

```
struct Container<T> {
    value: T,
}

impl Bark for Container<i32> {
  fn bark() {
    println!("bark");
  }
}

impl Bark for Container<String> {
  fn bark() {
    println!("bark");
  }
}
```
 
If instead of something like `Container<String>` we had `Container<impl Display>`, or some other trait bound specificed for the enclosed type, we could use conditional implementation to define a method just when the enclosed type satisfies the bound.

### Performance
At compile time, Rust fills in concrete types for the generic type parameters. This is called monomorphization and offers a performance benefit over the alternative method of filling in types at runtime.

### Orphan rule
In order to implement a trait on a type, you must have either the Type or the Trait local to your crate. This ensures that 2 different crates don't define conflicting Trait implementations on a dependency crate's type.

### Default implmentations
It's not possible to call the default implementation from an overriding implementation of that same method.

***Challenge*** Figure out how you would implmement something congruent to an override that calls 'super' in an OOP language.

Answer: Probably instead of trying to use the default implementation in your type-specific trait implementation, you should create a conditional implementation that implements the trait for types that implement another trait, or implements the trait for types that enclose types that implement another trait. Then you are essentially composing the behavior from the bounded trait in your implementation of the new Trait. 

### Trait Bounds
These are the same:

```
pub fn notify<T: Summary>(item: T)
```

```
pub fn notify(item: impl Summary)
```

```
pub fn notify<T>(item: T)
where
    T: Summary
```

#### Copy And Clone

When implementing a function with a generic parameter you will often get a compiler error that indicates you cannot move a non-copy type. You either have to borrow or add a trait bound for `Copy` to the argument.

_From Stack Overflow https://stackoverflow.com/questions/31012923/what-is-the-difference-between-copy-and-clone_

```
Clone is designed for arbitrary duplications: a Clone implementation for a type T can do arbitrarily complicated operations required to create a new T. It is a normal trait (other than being in the prelude), and so requires being used like a normal trait, with method calls, etc.

The Copy trait represents values that can be safely duplicated via memcpy: things like reassignments and passing an argument by-value to a function are always memcpys, and so for Copy types, the compiler understands that it doesn't need to consider those a move.

// u8 implements Copy
let x: u8 = 123;
let y = x;
// x can still be used
println!("x={}, y={}", x, y);

// Vec<u8> implements Clone, but not Copy
let v: Vec<u8> = vec![1, 2, 3];
let w = v.clone();
//let w = v // This would *move* the value, rendering v unusable.
```

### Conditional Implmentations
Implement a method for a type only when the enclosed generic type satisfies some trait bound:

```
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!(The largest member is x: {}", self.x);
        } else {

            println!(The largest member is y: {}", self.y);
        }
    }
}
```

#### Blanket Implementations

Conditionally implement a trait for any type that implements some other trait

```
impl<T:Display> ToString for T {
}
```
We would read this as, "Implement the ToString trait for any type that implements the Display trait".

#### Combining the two

Conditionally implment a trait for a generic type whenever the generic type's enclosed type implements some trait:

```
impl<T: Display> Bark for Container<T>
```

We would read this as: "Implement Bark for any container where the contained type implements Display"

### Lifetimes
Every reference has a lifetime, which is the scope for which that reference is valid. Usually inferred, we have to annotate lifetimes when the lifetimes of references could be related in a few specific ways.

We annotate the lifetime relationships between references using `generic lifetime parameters`. This ensures that the actual references are valid at runtime.

#### Dangling references
*A piece of data must have a longer lifetime than any reference to it*

```
    let y;
    {
        let x = 5;
        y = &x;
    }
    println!("y: {}", y);
```

This won't compile because x is dropped when the inner scope is over. The borrow checker knows x has a shorter lifetime than y (y the reference outlives x the value), so it throws an error.

```
{
    let x = 5;

    let y = &x;

    println!("y: {}", y);
}
```

Here x has a longer lifetime than y (y the reference does not outlive x the value) so no error.

#### Generic Lifetimes in Functions

```
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

Here, the borrow checker cannot determine whether the return value of #longest will be x or y. Therefore, it does not know the lifetime of the return value. It needs to know the lifetime of the return value, because it needs to check that the borrow (result) is not used after string1 or string2 go out of scope.

We satisfy this by constraining all the parameters and the return value with generic lifetime paramters:

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Here, the lifetime `'a` can be anything, as long as x, y, and the return value all have a lifetime at least as long as `'a`.

Specifically, we are guaranteeing that the reference that gets returned will live at least as long as the value referred to by x, _and_ at least as long as the value referred to by y, since the return value could reference either.

```
fn main() {
    let string1 = String::from("abcd");
    let result;

    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
    }

    println!("The longest string is {}", result);
}
```

In the above example, this code will not compile. The signature of #longest guarantees that
none none of the 3 references can be used after any of the others have gone out of scope. We have tied them together.

More specifically, we have specified that `result` will live at least as long as the shorter of `string1` and `string2`. It does not guarantee that it will live any longer than that. So the return value `result` is only valid in scopes where both string1 and string2 are valid. Since `result` is used after `string2` is out of scope, the borrow checker cannot validate the borrow of string2.

### Lifetime Annotations in Struct Definitions

when a struct holds a reference, its definition needs a lifetime annotation. This guarantees the reference in the struct will live at least as long as the struct.

***Challenge*** Create an example where a struct outlives a reference that is enclosed, and thus fails to compile.

Answer: This is done in #use_structs_containing_refereces in `chapter_10/src/main.rs`

### Lifetime Elision

_Lifetime Elision Rules_ are deterministic rules programmed into the compiler that allow it to infer lifetimes of references and thus not require annotations.

#### Input Lifetimes
Lifetimes on function or method parameters

#### Output Lifetimes
Lifetimes on return values

#### The Compiler's 3 lifetime inference rules
These three rules are applied to assign lifetimes to references. If the three rules are applied and the compiler still can't determine a lifetime, you must do so manually with annotations. If it can determine all the lifetimes, then they are _elided_

1. Each parameter that is a reference gets it's own lifetime.

```
fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
```

2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output parameters.
   
```
fn foo<'a>(x: &'a i32) -> &'a i32
```

This one is necessary because we can't return a reference to something that we created in the method, that would be a dangling reference. So we need to reference something that was passed in. In that case the returned reference has to live at least as long as the one passed in reference.

3. When there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime paramters. 

This one makes sense intuitively. Why would you want to use a reference returned by a method after the instance has gone out of scope?

#### Lifetime Annotations in Method Definitions

We need to declare lifetime names after the impl keyword and then use them after the struct's name, since they are part of its type.

`impl<'a> ImportantExcerpt<'a>`

### The Static Lifetime

Means the reference is valid for the entire lifetime of the program.

All string literals have the static lifetime:

```
let s: &'static str = "I have a static lifetime.";
```

Error messages that suggest using the static lifetime often result from attempting to create a dangling reference or from a mismatch of the available lifetimes.

### Syntax for Generic type parameters, Trait Bounds and Lifetimes all together

```
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Of course, instead of the `where` clause, we could also write it as:

```
fn longest_with_an_announcement<'a>(x: &'a str, y: &'a str, ann: impl Display) -> &'a str
```

or

```
fn longest_with_an_announcement<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str
```

## Chapter 11 (Writing Automated Tests)

*Attributes*: metedata about pieces of Rust code, such as `#[derive(_)]` or `#[test]`.

### Doc Tests
Doc Tests appear before a function as examples of its usage. You can compile and run these doc tests with `cargo test --doc`.

### Custom Failure Messages

`assert!`, `assert_eq!`, and `assert_ne!` can print custom failure messages that are input after the values that are being tested.

```
assert!(
    result.contains("Carol"),
    "The result did not contain name, value was  '{}'",
    result
);

assert eq!(
    4,
    result,
    "Expected value was {}, received {}",
    4,
    result
);
```

### Should panic
We can assert that a function call should panic.

```
#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(101);
}
```

We can also assert that the panic message contains a certain string.

```
#[test]
#[should_panic(expected = "guess value must be less than or equal to 100")] 
fn greater_than_100() {
    Guess::new(101);
}
```

### Testing options

Cargo test spits out a binary which is then run. To pass options to cargo test, specify them before the `--` separator. To pass options to the resulting binary, specify them after the `--` separator.

Similarily, view available `cargo test` commands with `cargo test --help`. View available flags to pass the binary with `cargo test -- --help`.

#### Specifying parallelism

`cargo test -- --test-threads=1`

#### Display print output from passing tests

`cargo test -- --nocapture`

#### Ignoring specific tests unless requested

```
#[test]
#[ignore]
fn expensive_test() {
    //slow running code
}
```

To run the ignored tests run `cargo test -- --ignored`

### Integration tests
To create integration tests, create a `tests/` directory at the top level of your crate. Each file under the `tests/` directory will be treated as its own crate. As such, you will need to import your crate in order to test it.

```
extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq(4, adder::add_two(2))
}
```

#### Running a specific integration test file

`cargo test --test integration_test`

#### Helper code for integration tests

Since each file under the `tests/` directory is treated as its own crate, and is run by the test harness, we need to put helper functions inside a helper inside a module within a new directory. Code inside a module inside a subdirectory such as `tests/common/mod.rs` will not be added to the test runner and can be included by any integration tests in `tests/`.

```
extern crate adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq(4, adder::add_two(2))
}
```

You cannot use integration tests to test binary crates that only have a `src/main.rs` and no `src/lib.rs`, because they do not expose anything to other crates. To use integration tests, keep your `src/main.rs` as simple as possible and have it shell out to `src/lib.rs`

## Chapter 12 (An I/O project: Building a Command Line Program)

### Reading Argument Values

`std::env::args` returns an iterator of the command line arguments.

When we use `std::env::args()` to gather the argument list, the first argument will always be the name of the binary. This is to follow a C standard that allow the program to know the name by which it was invoked.

#### Side note on importing modules

When importing, it is convention to bring the parent module into scope, so we `use std::env`. This allows us to use other methods from `std::env`, and avoids namespace issues and abiguity with other methods in the current module.

It also adds clarity when we call `env::args()` instead of just `args()`

#### Annotating types to create vectors

The compiler can't tell what we want to create with `#collect` when we are trying to use `#collect` to turn an iterator into a Vector. So we annotate it as follows:

`let args: Vec<String> = env::args().collect();`

alternatively we can use the _Turbofish_:

`let args = env::args().collect::<Vec<String>>();`

### Separation of Concerns for Binary Projects

The organizational problem of allocating responsibility for multiple tasks to the main function is common to many binary projects. As a result, the Rust community has developed a process to use as a guideline for splitting the separate concerns of a binary program when main starts getting large. The process has the following steps:

- Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
- As long as your command line parsing logic is small, it can remain in main.rs.
- When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

The responsibilities that remain in the main function after this process should be limited to the following:

- Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a run function in lib.rs
- Handling the error if run returns an error

### Idiomatic Rust

This chapter does a great job of illustrating the thought process of an idiomatic Rust programmer. Several times in a row it details the code smell and the incremental steps for improvement. The process of recognizing these smells, then making small improvement creates a chain of simple changes that greatly improves the maintainability and readibility of the program.

#### Extracting the Command Line Arg Parsing Logic

I try to summarize the dominoes that fall as we try to extract the command line arg parsing logic below:

1. We want to extract the command line parsing logic in preparation for moving it to    `src/lib.rs`. We implelment #parse_config method that returns a _tuple_ of config data. pg. 234.
2. We notice our #parse_config returns a _tuple_ of related data. Since the data is related, we can group it with more meaning in a `Config` struct. - pg. 235
3. Now we have the data in a `Config` struct, we notice that we have a method #parse_config() which has the sole purpose of creating a `Config` Struct. We can create an associated function on `Config` called `#new` to handle this in a more idiomatic way. pg. 236
4. We call `cargo run` and notice that when our user enters too few variables, we get an `index out of bounds` panic when trying to parse the command line args. We can give our users a better message to explain what they did wrong. In `#Config::new` we add our our condition to make sure there are enough command line args, and we `panic` with our own messaage if there are not. pg. 237
5. We notice that our `panic!` explains what the user did wrong but also offers them a backtrace and other extraneous information. We remember that a panic is more appropriate for a programming problem than a user error. We decide to return a result indicating success or error from `Config::new()`. pg. 238
6. The compiler tells us we need to update the callers of `Config::new()` to handle the new return type. We use `unwrap_or_else` to print the error message returned by `Config::new()` and call `process::exit(1);` to indicate the program terminated with an error state. pg. 239

##### Summary

- Tuples should often be converted to structs so that the the named fields can convey more meaning.
- If a method is just creating a struct, or taking some other action usually handled by an associated function. We should just create `#new` or some similar associated function in the `Struct` impl.
- Panics meant for the programmer (Usually from some other module or std lib method) are not helpful for the user.
- Panics in general (including the ones we define) are not great for the user, prefer to use `Result` to convey `Err` conditions.

#### Extracting Logic From Main.

1. We want to extract a function named run that will hold all the logic currently in the main function. pg 240
2. We see that there is still a call to `expect` in `#run`, we would like the program not to panic, so we decide to have `#run` return a `Result<(), Box<dyn Error>>` in preparation for letting `#main` handle the error case in a user friendly way. pg. 241
3. The compiler tells us with "unused `Result` that must be used" warning that we still have to handle our error case in main. So we add a `println!` that says there was an "Application error", prints the error and then exits the process with a code of `1` to indicate the program ended in an error state. pg. 242

##### Summary

- Calls to `#expect` and `#unwrap` are indicators that your code could have more verbose error handling.
- If you see an unused "`Result` that must be used" warning then you are not handling an error case.

#### Splitting Code into a Library Crate

1. We want to move all the code that isn't the main function from `src/main.rs` to `src/lib.rs` pg. 242
2. We make `src/lib.rs` and copy/paste everything over. We change `run()`, `Config`, `Config::new` and all of `Config`'s fields to public to give our crate a public API. pg. 243
3. The compiler tells us we need to bring our API into scope to be used in `src/main.rs`, so we add `extern crate minigrep` to include the crate and appropriately instantiate `Config` and `#run`. pg. 243

#### Using Test-Driven Development
1. We want to have a `#search` function return all the lines from the file that contain the query. So we write a test that asserts that `#search` returns the line from a string that contains a given query. pg. 245
2. The compiler tells us `#search` is not found. We add a naive `#search` function that just returns an empty vector, just to get the test to compile. pg. 245
3. The compiler tells us "missing lifetime specifier", because `#search` returns a `Vec` of borrowed `str`s, but the compiler cannot tell if the borrow comes from `query` or `content`. We add a `'a` lifetime annotation to `content` and the return type because the return vector will always be made up of values borrowed from `content`. 
4. We see that the test compiles but fails--it only returns an empty `Vec<>`. 
5. We add code to handle the 1-match use case. The test passes.
6. We add a test to test the multiple-match use case. The new test fails. We add code to handle the multiple-match use case. The test passes.
7. We notice we are running a for loop on an iterator just to return a new collection of selected items from the iterator. We refactor into a call to `#filter`.

#### Summary

- When we write a function and get a "missing lifetime specifier error", we probabably just have to tell the compiler which arguments the return value will be borrowing from.
- `#filter` is more expressive way to trim down a vector of references (or owned values for that matter) than using a for loop.

***Challenge*** Write an integration test that calls run with a filename, and then watches stdout to make sure the correct values are printed.

-> Finish in `chapter_12/tests/run_tests.rs

#### Adding a case-insenstive search function

1. We want to allow users to have a case-insenstive search option so we add a test for a method called `#case_insensitive_search`. pg 250
2. We implement a skeleton method to get the test to compile. It fails. pg 251
3. We copy/paste the original `#search` method. We call `.to_lowercase` on both `query` and `line` so that we can compare each line to the query in a case-insensitive manner.
4. We get a compiler error: "expected an implementor of trait `Pattern<'_>` help: consider borrowing here: &query.to_lowercase()", `.to_lowercase` creates a new String, so we need to borrow here again since `contains` expects a borrow. We add the & in front of `query` and the test passes.

#### Summary
- str manipulation usually results in the creation of a new String.

#### Adding a environment variable for case-insenstive search
1. We add a configuration option to the `Config` struct for case-insensitive search. pg. 252
2. We add an if/else block to `#run` to call either `search` or `case_insensitive_search` based on the `Config.case_sensitive` value. pg. 253
3. The compiler tells us we need to update the other instantiations of `Config` to include a `case_sensitive` value. We update the `Config::new` function to check the env variables using `std::env::var("CASE_INSENSITIVE")` and set the `case_sensitive` field.
4. The compiler gives us a mismatched types error because `#std::env::var` returns a `Result`, not a `bool`. We use `#is_err` to convert the `Result` to a `bool`

#### Summary
- `#is_err` is useful for converting a `Result` into a `bool`

#### Printing error messages to the stderr and successful output to stdout
We can use `eprintln!` to print errors to stderr instead of stdout.

#### #unwrap_or_else
When the `Result` is an `Ok` value, `#unwrap_or_else` will just unwrap the `Ok` value. When it is an `Err` value, it will allow us to handle the error with a closure that takes the value held by the `Err` variant as an argument:

```
let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
});
```

Remember that `unwrap` just panics when a `Result` is `Err` or an `Option` is `None`
Remember that `expect` panics when `Result` is `Err` or an `Option` is `None` and lets you set your panic message.

#### Trait Objects
When we return `Result<(), Box<dyn Error>>`, The `Box<dyn Error>` is a _Trait Object_. Trait objects let us return any type that implements a given trait. `Error` in this case. But we don't have to specify which particular type the return value will be.

*Question* Why use a trait object instead of a trait bound such as `impl Error` on the return type? 

Answer: We have to do this often when using the `?` operator to return an error because not every error type has an implementation of `From<_>` for `impl std::error::Error`. Basically the compiler does not know how to change our error into the more general error.

*Challenge* Control case insensitivity through either a command line argument or an environment variable. Deide which should take precedence if both are set.

-> Finished: See `Config::is_case_senstive` in `src/lib.rs`