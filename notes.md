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

# Chapter 12 (Iterators and Closures)

closure

: Rust's closures are anonymous functions you can save in a variable or pass as an argument to other functions. Unlike functions, closures can capture values from the scope in which they're called.

Closures cannot be exposed in the public API of our crates so they do not require explicit type annotations for their arguments and return values.

#### Fn traits
Provided by the standard library, all closures implmement at least one of the traits: `Fn`, `FnMut`, or `FnOnce`.

```
struct Cacher<T>
    where T: Fn(u32 -> u32)
{
    calculation: T,
    value: Option<u32>
}
```

Cacher is a struct that can hold any closure that has one u32 argument and return a u32.

**Challenge** Extend the Cacher impl furthur to support non-copy types

#### Capturing the environment with closures

**Question** What does it mean that "closures can capture values from the scope in which they are called"

```
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;
    
    assert!(equal_to_x(y))
}
```

Answer: It can actually take a the value of a variable that is in scope with it. It does this in three ways:

1. Immutably borrowing
2. Mutably borrowing
3. Taking ownership. (move)

Closures will preferentially capture variables by reference and only move down the above list as needed.

When writing functions that take closures as arguments, we just manually annotate the `Trait` bounds:

1. Fn - the closure can only capture values by immutable reference (&T) 
2. FnMut - the closure can capture by immutable or mutable reference (&mut T)
3. FnOnce - the closure can capture by immutable or mutable reference, and consume values it has taken ownership of (drop or give ownership to some other context).

The compiler will capture each variable in the least restrictive manner possible on a variable by variable basis.

`move` keyword forces closures to take ownership of the values it uses from the environment **on creation**. Useful for passing a closure to a new thread and having it take some piece of data with it.

```
fn main() {
    let x = vec![1,2,3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}, x);

    let y = vec![1,2,3];

    assert!(equal_to_x(y));
}
```

Here because x is moved into the closure, we cannot use it after the definition.

*Example*: Trying to pass a closure that is `FnMut` to a method that is restricted to `Fn`

```
fn apply_to_3<T: Fn(i32) -> i32>(f: T) ->  {
    f(3)
}

fn main() {
    let y = 5;

    let double = |x| {
      y = 10;
      2 * x
    };

    apply_to_3(double);
}
```

Here we will get an error: "expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`" because the compiler infers the closure is FnMut, since it mutates y. 

*Example*: Trying to pass a closure that is `FnOnce` to a method that is restricted to `Fn` or `FnMut`

```
use std::mem;

fn apply_to_3<T: FnMut(i32) -> i32>(f: T) ->  {
    f(3)
}

fn main() {
    let y = "foo".to_string();

    let double = |mut x| {
      mem::drop(y);
      2 * x
    };

    println!("3 doubled: {}", apply_to_3(double));
}
```

Here we will get an error: "expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce`" because the compiler infers the closure is FnOnce, since it destroys y. Another way the closure could have moved y out of _its_ context, would be to return the value it moved from the outer context:

```
let y = "foo".to_string();
let double = |x| y;
```

Note for this closure to be assigned only `FnOnce`, y must not implement `Copy`.

*Question* Why can `FnOnce` closures only be called once?

As seen the previous example, closures that only satisfy `FnOnce` (and don't satisfy `Fn` or `FnMut`) have to take ownership of a value in their environment because they are in turn going to move it somewhere else, and this can only be done once.

If we update `double` to only satisfy `FnOnce`, we can see that trying to pass our closure twice will result in a "use of moved value" error:

```
use std::mem;

fn apply_to_3<T: FnOnce(i32) -> i32>(f: T) ->  {
    f(3)
}

fn main() {
    let y = "foo".to_string();

    let double = |mut x| {
      mem::drop(y);
      2 * x
    };

    println!("3 doubled: {}", apply_to_3(double));
    println!("3 doubled: {}", apply_to_3(double));
}
```

Similarily if we try to use it twice where it was defined:

```
use std::mem;

fn main() {
    let y = "foo".to_string();

    let double = |mut x| {
      mem::drop(y);
      2 * x
    };

    double(3);
    double(3);
}
```

*Question*: How do we use an `FnMut` closure twice?

```
fn apply_to_3<T: FnMut(i32) -> i32>(mut f: T) -> i32 {
    f(3)
}

fn play_with_closures() {
    let mut y = 5;

    let mut double = |x| {
        y = y + 10;
        2 * x
    };

    println!("{}", apply_to_3(&mut double));
    println!("{}", apply_to_3(&mut double));
}
```

Answer as detailed from the above:
1. The captured variable has to be mutable so the closure can mutate it.
2. The closure also has to be `mut` because an `&mut` is stored inside.
3. The closure can only be passed as a `&mut`, the borrow is because we want to use it again in this context. Since it will be modified when `#apply_to_3` uses it, the `mut` is necessary too.
4. Any function borrowing the mutable closure needs to annotate that argument as mutable.

If we want to allow the closure to be moved and then used twice, we can do something like this:

```
fn apply_to_3<T: FnMut(i32) -> i32>(mut f: T) -> i32 {
    f(3);
    f(3)
}

fn play_with_closures() {
    let mut y = 5;

    let double = |x| {
        y = y + 10;
        2 * x
    };

    println!("{}", apply_to_3(double));
}
```
Note that we don't need to pass `#double` as a borrow because we are letting `#apply_to_3` take ownership. Also we don't need `#double` to be mutable, because it will be defined as mutable when it gets moved to `#apply_to_3`. Furthermore, `#play_with_closures` can't use `#double` again since it was moved.

**Also worth note:**

As the closure definition determines what Fn trait the closure will be assigned, the closure definition is also solely responsible for determining if the closure can be used twice. Even if we pass it to a function that allows `FnOnce`, we can still use it multiple times:

```
fn play_with_closures() {
    let mut y = 5;

    let mut double = |x| {
        y = y + 10;
        2 * x
    };

    println!("{}", apply_to_3(&mut double));
    println!("{}", apply_to_3(&mut double));
}
```

**A tip**

Most of the time it makes sense to start by specifying your closure arguments as `Fn` and the compiler will tell you if you need to change the bound to `FnMut` or `FnOnce`.

**Also worth note:**

The compiler only looks at the body of the closure to implement the `Fn`, `FnMut` or `FnOnce` traits for a closure. The `move` keyword does not force a closure to only satisfy `FnOnce`.

This is because the `move` keyword only makes the closure take ownership of the variables when the closure is created!. The closure can still be `Fn` as long as it doesn't modify the variables, and it can still be `FnMut` as long as it does not move the variable _again_, giving ownership to some other context, often seen when a captured variable is in turn dropped by the closure or returned from the closure.

This is best explained [here](https://stackoverflow.com/questions/50135871/how-can-a-closure-using-the-move-keyword-create-a-fnmut-closure)

### Iterators
Rust `iterators` (which `impl Iterator`) are _lazy_. They have no effect until you call methods that consume the `iterator` to use it up. That is, no iteration takes place until you call some other method on the `iterator`.

#### A look under the hood

```
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

Here, `Item` is an _associated type_, it means, among other things, that implementing the `Iterator` type requires that you also define an `Item` type. The `Item` type is used as the return type of the `#next` method.

##### next

```
#[test]
fn iterator_demo() {
    let v1 = vec![1,2,3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

Here we need v1_iter to be mutable because calling `#next` changes iternal state (the pointer that keeps track of where we are in the iterator). Methods that call `#next` are called _consuming adapters_ because calling them uses up the iterator. 

```
let v1_iter = vec![1,2,3].iter();
assert_eq!(v1_iter.sum(), 6);

//
// v1_iter.next();
```

`#iter`: produces an iterator over immutable references.
`#iter_mut`: produces an iterator that lets us iterate over mutable references, requires the collection we begin with is mutable.
`#into_iter`: Moves ownership and returns owned values.

_Iterator adapters_ allow you to change iterators into different kinds of iterators. Because iterators are lazy, you have to call a _consuming adapter_ at to get the results from calls to _iterator adapters_.

```
let v1: Vec<i32> = vec![1,2,3];

v1.iter().map(|x| x + 1);
```

This will result in a warning: "unused `std::iter::Map` which must be used: iterator adaptors are lazy and do nothing unless consumed".

Commonly we will finish with the `#collect` consuming adaptor:

```
let v1 = vec![1,2,3];

v1.iter().map(|x| x + 1).collect::<Vec<_>>();
```

Notice the underscore in the _turbofish_ lets us allow the complier to infer the `Item` type we want the resulting collection to have.

### Custom Iterators
See `chapter_13/src/counter.rs`

*note* The iterator constructor methods `#iter`, `#iter_mut`, and `#into_iter` convert collection types like `Vector` into `std::slice::Iter`, `std::slice::IterMut`, and `std::slice::IntoIter` structs, respectively. Each implement the `Iterator` trait. When  you implement `Iterator` for your own custom type, there is no need to call any constructor methods as your type is already any iterator.

### Zip
`#zip` returns `None` when either of its input iterators returns `None`

```
    let v1 = vec![1, 2, 3, 4];
    let v2 = vec![1, 2, 3];

    let pairs = v1.into_iter().zip(v2).collect::<Vec<_>>();

    assert_eq!(pairs, vec![(1, 1), (2, 2), (3, 3)]);
```

### Upgrading our minigrep program from Chapter 12

1. We want to avoid calling `#clone` in `chapter_12/src/lib.rs#Config::new`, we recognize it would be nice if `#new` owned the command line args that `#main` passes in.
2. We edit main to pass in the args directly, instead of collecting the args into a vector and borrowing, since `#std::env::args` returns Args, which is an iterator already.
3. The compiler tells us the type of the args argument needs to change on `#new`, since it was taking a String slice but being given `std::env::Args`. We change the signature of `#new` to accept `std::env::Args`. Because we'll be mutating args by iterating over it with `#next`, `#new` is now considered a _consuming adapter_ and has to have mutable ownserhip of the iterator, so `Args` is also annotated as `mut`. 
4. The compiler tells us we cannot index into a value of type `Args` (`let query = args[1].clone();`). We use `#next()` to get each command line arg in order.
5. A test fails with an error: "not enough arguments. We remove the argument length validation because we are checking that each specific argument gets parsed with `#next()`.

*Question* If we are taking ownership of the iterator are we also taking ownership of the `Vec` that produced it?

Yes.

*Question* When does iterating require a mutable reference because the type that is `Iterator` is modified internally by calling next? This didn't seem to be the case with the examples in `chapter_13/src/main.rs#use_iterator_constructors`.

Answer: Any method that calls `#next()` on an `Iterator` is mutating and thus consuming it. _Consuming adapters_ like these must take ownership of the `Iterator` and declare it `mut`. We do this manually in `chapter_12/src/lib.rs#run`, but it happens internally in methods like `#for_each` and `#sum`.


### Upgrading #search

1. We want to use iterator adapters to be more verbose than using for loops, and to eliminate intermediate mutable collections (functional programming!), which can be mutated haphazardly by future developers and cause bugs. It could also allow parallelism in the future.
2. We modify `chapter_12/src/lib.rs#search` to use `#lines` and `#filter`

#### Summary

- If we just want a subset of a collection, `#filter` is our fixer.
- If we want and iterator that splits a str by `\n`, `#lines` is our fixer.

*note* Because the compiler will hide dereferences of borrows, the `&` operator can often be dropped:

`#contains` is defined on str
```
content
    .lines()
    .filter(|line| line.contains(query))
    .collect()
```

So our intution is to do the following:
```
content
    .lines()
    .filter(|&line: &str| line.contains(query))
    .collect()
```

but we see with annotations what happens in the background:
```
content
    .lines()
    .filter(|line: &&str| line.contains(query))
    .collect()
```

Line began as a slice of a slice, but the conversion happens behind the scenes.

### Zero-Cost Abstractions

Iterators are one of Rust's _zero-cost abstractions_, meaning using the abstraction imposes no additional runtime overhead. The iterator code gets compiled down to roughly the same code as if you'd written the lower-level code yourself.

### Unrolling

When using an iterator to iterate over a fixed length array, the compiler knows that it can _unroll_ the loop. This means it removes the loop-controlling code. The assembly doesn't have to worry about incrementing the index, or checking the loop bounds. The compliler just repeats the code for each iteration with the array elements hardcoded in place. Often times _unrolling_ allows values in the original array to be stored in registers on the processor, which is a big performance gain!

Rust often does this automatically with both `Iterator` solutions and `for` loops solutions. There are facilities available for forcing/disabling this behavior.

## Chapter 14 (More about Cargo and Crates.io)

### Release Profiles

_Release profiles_ are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code.

The two main release profiles are _dev_ (`cargo build`) and _release_ (`cargo build --release`). 

### Overriding a Release Setting:

```
[profile.dev]
opt-level = 1
```

### Publishing a Crate to Crates.io

#### Documentation Comments
Creating comments using three slashes (`///`) immediately before an item allows us to add documentation that will be generated as HTML. We can use markdown to style the comments. 

Generate the HTML documentation by running `cargo doc`. This will run the _rustdoc_ tool and put the generated HTML in `target/doc`.

`cargo doc --open` will build the HTML for the current crate's documentation and open in the browser.

Sections that commonly appear in docs for a function:

- `# Examples` - Gives one or more code examples often with an assert.
- `# Panics` - Details the scenarios that could cause a panic.
- `# Errors` - When the function returns `Result`, the types of errors and the conditions that cause them are often detailed.
- `# Safety` - If the function is _unsafe_ to call, it should be explained why it is unsafe and details the invariants the callers are expected to uphold.

#### Documenation Comments as Tests
Any code appear between triple backticks will be run as documentation tests when we run `cargo test`. We can run just the doc tests with `cargo test --doc`.

But we can only write documentation tests for `pub` functions (the testing harness won't find them otherwise, private functions can only be called directly from unit tests).

If we don't want setup code from documentation tests to make it into our HTML examples, we can hide it with `#`:

```
/// ```
/// # fn foo() // this function will be hidden when we run "cargo doc"
/// println!("Hello, world!");
/// ```
```

We can escape the `#` that would usually hide a line by repeating it:
```
/// let s = "foo
/// ## bar # baz"
```

#### Commenting Contained Items
We can use `//!` to comment the item that contains the comments instead of the item that immediately follows the comments. This is useful for documenting an entire crate or module.

### Re-Exporting with `pub use`
When desigining your crates's API it can be useful to re-export publilc items that are nested deep in your crate with `pub use`. This will make it appear as though your item were declared wherever you re-exported it. 

Suppose within your `art` crate, you have a `kinds` module with a `PrimaryColor` type. Another crate could use the `PrimaryColor` type by calling `use::kinds::PrimaryColor`. Or if you wanted them just to be able to call `art::PrimaryColor`, then you can re-export `PrimaryColor` by adding `pub use kinds::PrimaryColor` to your `src/lib.rs`.

This will also add the re-exported items to a `Reexports` section on your crate's doc page.

### Publishing a Crate

You can set the name of your crate in `Cargo.toml` like so:

```
[package]
name = 'Guessing Game"
```

Names are allocated first-come first-serve and must be unique.

**!PUBLISHED CODE CANNOT BE DELETED!!!**

*If you accidentally upload any secrets they must be reset immediately!*

### Cargo Workspaces
_Workspaces_ are useful for further splitting larger crates into multiple library packages

A _workspace_ is essentially a set of packages that all share the same `Cargo.lock` and output directory.

We can create a _workspace_ by adding a Cargo.toml to a directory with the a `[workspace]` section and add `members` as follows:

```
[workspace]

members = [
    "adder",
]
```

We can then run `cargo new --bin adder` to create a binary crate inside our workspace. it will be a normal crate with it's own `cargo.toml` and `src` directory inside of `adder/`. It will share the `cargo.lock` and `/target` in the root with the other crates in the workspace.

We can add another member create to the workpace as follows:

```
[workspace]

members = [
    "adder",
    "add-one",
]
```

and generate it with: `cargo new add-one --lib`.

After we add some code to the add-one library, we can make the `adder` binary depend on it by modifying `adder/src/Cargo.toml`:

```
[dependencies]
add-one = { path = "../add-one" }
```

we can then pull in code from `add-one` in `adder`:

```
extern crate add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
```

and build and run it from the top level of the workspace:

```
cargo build
```

```
cargo run -p adder
```

the `-p` specifices which package in the workspace we want to run. The package name is of course defined by the `Cargo.toml` under that member crate's directory.

#### Depending on an External Crate in a Workspace
Because all the workspace members share a root `Cargo.lock`, we can make sure they all depend on the same versions of all dependencies.

We can add an external dependency to `add-one/`:

```
[dependencies]

rand = "0.3.14"
```

and then use it in `add-one/` with `extern crate rand`. Note we still need to add the dependency to other crates' `Cargo.toml` if we want to use it there. If we add the same dependency line to `adder/`, then cargo will add `rand` to the list of dependencies for adder in `Cargo.lock`, but since we are using the same version in both places, it will not have to download `rand` again.

#### Testing within a workspace

Running `cargo test` in a workspace will run all the tests for all the crates in the workspace. So in our case, `cargo test` runs the unit tests in our lib crate, `add_one/`, then checks for unit tests in `adder/`, but doesn't find any, and then runs the documenation tests in `add_one/`. It doesn't check for documentation tests in `adder/` because it is a binary crate, and cannot have any documentation tests.

We can use the `-p` flag to specify which package's test we would like to run with `cargo test`: `cargo tests -p add_one`.

#### Publishing crates from a workspace
`cargo publish` does not have a `-p` (package) flag or an `-all` flag, so to publish crates from a workspace, you will have to run `cargo publish` from each crate's individual create directory.

### Cargo Install
`cargo install` can be used to install crates from `crates.io` that have binary targets (those with a `src/main.rs` or other file specified a binary). All binaries installed with `cargo install` are stored in the installation root's _bin_ folder. If Rust was installed with `rustup` then this will be _$HOME/.cargo/bin_.

Install goes like this:

```
cargo install ripgrep
```

View binaries added with cargo install: `ls ~/.cargo/bin`.

### Extending Cargo with Custom Commands
If a binary in your $PATH is named `cargo-something` you can run it as if it were a build-in cargo subcommand by running `cargo something`.

View custom binaries installed with cargo: `cargo install --list`. Any of these that begin with `cargo-` and be run as though they were cargo built-ins.

## Chapter 15 (Smart Pointers)

While references borrow the value they point to, they don't have any special capabilities other than referring to data, nor do they have any overhead. _smart pointers_ are data structures that also act like a pointer, but have additional metadata and capabilities. Unlike _references_ which are pointers that only borrow data, _smart pointers_ often own the data they point to.

The _reference counting_ smart pointer enables you to have multiple owners of data by keeping track of the number of owners, and when no owners remain, cleaning up the data. Remember that usually there can only be one owner of a piece of data, many immutable references, and only one mutable reference.

`String` and `Vec<T>` count as smart pointers because they own some data and allow you to manipulate it. They also have metadata (such as their capacity) and extra capabilities (in the case of `String`, all items must be UTF-8).

### Implementation of Smart Pointers
Smart pointers are usually implemented using structs. The characteristic that makes a smart pointer a smart pointer and not just a struct is that it also implements `Deref` and `Drop`.

`Deref` allows a type to behave like a reference. This allows us to implement code that works with references or smart pointers.

`Drop` allows us to customize what happens when an instance of the type goes out of bounds.

#### Common Smart Pointer Types

- `Box<T>` is used for allocating values on the heap
- `Rc<T>` is the _reference counting_ smart pointer, which allows a value to have multiple owners.
- `Ref<T>` and `RefMut<T>` accessed through `RefCell<T>` a type that enforces the borrowing rules at runtime instead of compile time.

**Question**: What does it mean That `Ref<T>` and `RefMut<T>` are accessed through `RefCell<T>`? Does that mean that they have to be enclosed in `RefCell<T>`? If so, why?

Answer: When we have `RefCell<T>` we use `#borrow` and `#borrow_mut` methods to get `Ref<T>` and `RefMut<T>` smart pointers, which will let us reference or mutate the data enclosed in the `RefCell<T>` as we would with a normal `&T`, or `&mut T` reference. The difference is these borrows are checked at runtime instead of compile time.

_Interior mutability_ is when an immutable type exposes an API for mutating an interior value. 

### Using `Box<T>` to Point to Data on the Heap
You can use `Box<T>` to put data on the _Heap_ instead of the _Stack_. All that will remain on the _Stack_ is the pointer. The _Heap_ allocation of the enclosed data is really the only performance overhead.

Why would you want to use a `Box<T>` then?

- When you have a type whose size can't be known at compile time but you want to use a _value_ of that type in a context that requires an exact size.
- When you have a large amount of data and you want to transfer ownership but you want to make sure the data will not be _copied_ when you do so.
- When you want to own a value and you only care that it's a type that implements a certain trait rather than being a specific type.

**Question** What is a context for which you would need to know the exact size of a value, other than compilation, and why would putting it on the heap help?

Answer: This question results from a slight misunderstanding of the first bullet above. The "context that requires an exact size" is referring to a place in your code where the compiler will assert that the size of the type is known.

The best example of such a context is recursive types. You cannot know the size of the value bound to the recursive field at compile time, so you could define your type to reference the enclosed type inside of a `Box<T>`, which has a known size.

**Question** Can you not own a value that has no concrete type assigned at compile time? Why?

Answer: Trait objects will get into this: page 369. **Follow up**: are trait objects only necessary in collections?

### Recursive Data Types
When we try to implement a recursive type in Rust:

```
enum List {
    Cons(i32, List),
    Nil
}
```

We will get a compiler error: 

```
error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:3:1
  |
3 | enum List {
  | ^^^^^^^^^ recursive type has infinite size
4 |     Cons(i32, List),
  |               ---- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
  |
4 |     Cons(i32, Box<List>),
  |               ++++    +
```

Because `List` is recursive (it can hold another `List` directly). The compiler cannot tell how much space it needs to store a `List`. 

**Question** Why does Rust need to know how much space it needs to store a value of a given type at compile time?

Possible answer: When it is going to request memory to be allocated on the heap, it needs to give Malloc a size. Additionally, to index into a `Vec<T>`, or just to access a field of a struct bound to that type. 

**Follow Up**: It sounds like for a Vector, both the length and capacity are maintained. The length is the amount of memory the object is currently using, while the capacity is the total amount that has been received from the operating system. So to throw an index out of bounds error, rust would have to determine the length of the `Vec` by dividing the length value it has stored by the size of the contained type. 

#### Determining the Space Needed to Store a Non-Recursive Data Type

```
Enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
In the case of an enum, since only one variant is used, the compiler determines that the most space an enum will need is the space needed to store its largest variant. In the case of `Message`, this is `ChangeColor`.

#### Back to Recursive Types

So in the case of a _recursive type_, the compiler can't determine the space needed to store the type. It would loop infinitely if it tried.

To get around this we can use a pointer to point to the next item in the recursive variant. This way the compiler will always know how much space is needed to store a value of the recursive type.

```
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

Alternatively we could use a reference instead of a `Box<T>`, but then we would need lifetime paramters since we would then be borrowing the next List.

```
enum List<'z> {
    Cons(i32, &'z List<'z>),
    Nil,
}
```

Here the 3 lifetime parameters denote that the List pointed to inside of `Cons` will live at least as long as the reference to it, and the reference inside of `Cons` will last at least as long as the `List`.

### Treating Smart Pointers Like Regular References with the Deref Trait

#### Box<T> vs &T

In contrast to references, boxes own the values they point to:

```
    let x = "Foobar".to_string();
    let y = &x;

    println!("x: {}", x)

    let a = "Foobar".to_string();
    let b = Box::new(a);

    // borrow after move
    // assert_eq!("Foobar".to_string(), a);
```

#### Implementing our own Smart Pointer.

We can't use the deref operation `*` without implementing the `Deref` trait for our type:

```
pub trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}
```

Here we see `Deref` needs us to define an associated type "Target", which in the case of Deref, specifies the type the deref operator `*` will return.

We also have to implement the method `#deref` to determine how the deref operator will retrieve the "Target" value the pointer points to.

```
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
```

Notice the return type of `#deref`. When the compiler deferences smart pointers, it actually calls `#deref` to get a reference that it knows how to dereference, and then uses * to retrive the referenced value as it normally would.

For example:

```
    let x = "Foobar".to_string();
    let y = Box::new(x);

    assert_eq!("Foobar".to_string(), x);
    assert_eq!("Foobar".to_string(), *y);
```
Here we can use `*` on `y`, because it is a box and implements `Deref`. The compiler changes our dereference of y to this:

```
    assert_eq!("Foobar".to_string(), *(y.deref()));
```

The reason `#deref` returns a reference to `T` is because we want smart pointers like `Box` to retain ownership of the value they point to after they are dereferenced.

**Deref Coercion** is a convenience that Rust performs on arguments to functions and methods. A reference to a type that implements `Deref` is converted into a reference to a type that `Deref` can convert the original type into.

#### Deref Coercion In Action:

```
fn hello(name: &str)  {
    println!("Hello, {}!", name);
}


fn main() {
    let m: MyBox<String> = MyBox::new(String::from("Rust"));
    hello(&m);

    // &MyBox(String::from("Rust"));
    // *(&(MyBox(String::from("Rust")).deref())); // calls `deref` on MyBox<String>
    // *(&(&String::from("Rust")))                // `deref` returns a reference to the String stored in the MyBox<String>
    // &String::from("Rust")                      // & and * cancel
    // "Rust": &str                               // calls `deref` on &String, leaving us with &str
}
```

Here, the complier will see that we have a `&MyBox<String>` and are passing it as a parameter to a function that expects a `&str` argument. Since the parameter and the argument do not match, it checks if the parameter is a reference to a type that implements `Deref`. Since it is, it calls `deref` on the `MyBox` and ends up with a `&String`. Then it calls `deref` again to turn the &String into a &str.

This is what it would look like without _deref coercion_:

```
fn main() {
    let m = MyBox::new(String::from("RUst"));
    hello(&(*m)[..])
}
```
Here we would have to use `*` to deref a `MyBox<String>` into a `String`, then use `&` and `[..]` to take a string slice of the entire `String`.

The compiler will call `deref` as many times as necessary to make the paramter's type match the function argument's type.

Remember that _deref coercion_ only works when the passed in argument is a reference to a type that implements `Deref`.

```
error[E0308]: mismatched types
  --> src/main.rs:47:11
   |
47 |     hello(x);
   |           ^
   |           |
   |           expected `&str`, found struct `MyBox`
   |           help: consider borrowing here: `&x`
   |
   = note: expected reference `&str`
                 found struct `MyBox<String>`
```

This is to avoid implicitly creating references.

### Deref Coercion and Mutability
Similar to how we use the `Deref` trait to override the `*` operator on immutable references, we can use the `DerefMut` trait to override the `*` operator on mutable references.

There are three cases where Rust does _deref coercion_:

- From `&T` to `&U` when `T: Deref<Target=U>`
- From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
- From `&mut T` to `&U` when `T: Deref<Target=U>`

In the third case we can make an immuatable reference from a mutable one without breaking the borrowing rules. When there is a mutable reference, it must be the only reference. So we can destroy a mutable reference for an immutable one. However, we cannot create a mutable reference from an immutable one, because that would require that there was only one immutable reference, and that cannot be guaranteed.

We saw an example of the first case above. The second case would occur if we had passed a `&mut MyBox<String>` and it was coerced into a function requiring a `&mut str`. The third case would occur if we had passed a `&mut MyBox<String>` into a function requiring a `&str`.

### The `Drop` Trait

The functionality of the `Drop` trait is almost always used with smart pointers. In the case of `Box<T>`, Drop::drop is used to deallocate the space on the heap that the `Box<T>` was pointing to.

Variables are dropped in the reverse order of their creation.

#### Dropping a value early.
 We can't call `Drop::drop` early manually:

```
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");

    c.drop();
    println!("Custom Smart Pointer dropped before the end of #main.");
}
``` 

This results in a compiler error:

```
error[E0040]: explicit use of destructor method
   --> src/main.rs:107:7
    |
107 |     c.drop();
    |     --^^^^--
    |     | |
    |     | explicit destructor calls not allowed
    |     help: consider using `drop` function: `drop(c)`
```

This is because rust will still automatically call the destructor at the end of the variable's scope even though we called it early, which would result in a double free error.

#### `#std::mem::drop`

The proper way to do this is to call `#std::mem::drop`, which is included in the prelude as `#drop`. `drop` will end the variable's scope _and_ call `Drop::drop` to clean up.

```
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");

    drop(c);
    println!("Custom Smart Pointer dropped before the end of #main.");
}
``` 

### `Rc<T>`, the Reference Counting Smart Pointer

`Rc<T>` is used to enable multiple ownership. `Rc<T>` keeps track of the number of references to a value to determine if a value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

We use `Rc<T>` when we want to allocate some data on the heap for multiple parts of our program to read and we can't determine at compile time which part will finsih using the data last. If we knew which part would finish last, we could just make that part the data's owner.

`Rc<T>` is only for single threaded scenarios.

#### Trying to Implement Multiple Ownership with `Box<T>`

```
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```

This will result in an error: "use of moved value: `a`" because `Box<T>` owns its enclosed value.
  
#### Implementing Multiple Ownership with `Rc<T>`

```
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
```
Here we will use `#Rc::clone` to create subsequent references to our List. This will let multiple variables share ownership of the data in the `Rc<List>`. We will do this twice when creating `b` and `c`. Each time we call `Rc::clone`, the count of references (reference count) to data within the `Rc<List>` will be incremented.

Notice that we need the original owner of the `List`, `a`, to hold the `List` via an `Rc<T>` so that it can be included in the count of references.

Remember that calling `#Rc::new` takes ownership of its enclosed value:

```
let a = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));

{
    let alist = Rc::new(a);
    let b = Cons(3, Rc::clone(&alist));
    let c = Cons(4, Rc::clone(&alist));
}

// borrow after move
// println!("a: {:?}", a);
```

`#Rc::clone` doesn't make a deep copy, as most implmentations of `Clone` do. The call to `#Rc::clone` only increments the reference count. Because of this, Rust convention says to use `Rc::clone(a)` instead of its equivolent, `a.clone()` to distinguish between usages of `Rc<T>`'s shallow copying `clone` and other type's deep copy implementation of `clone`.

### Interior Mutability

_interior mutability_ is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data. normally this is disallowed by the borrowing rules. To mutate data, the pattern uses unsafe code inside a data structure to bend Rust's usual rules that govern mutation and borrowing.

We can use types that use the _interior mutability_ pattern when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can't guarantee that. The unsafe code involved is then wrapped in a safe API, and the outer type is still immutable.

#### Enforcing borrowing Rules at Runtime with `RefCell<T>`

Unlike `Rc<T>`, `RefCell<T>` represents single ownership over the data it holds.
Unlike `Box<T>`, the borrowing rules are enforced at _runtime_, not compile time

Checking the borrowing rules at compile time allows us to catch errors sooner, and avoid any performance impact of the checks on our running program.

Checking the borrowing rules at runtime allows certain memory-safe scenarios that can't be checked by the compiler. The `RefCell<T>` type is useful when you're sure your code follows the borrowing rules but the compiler is unable to guarantee that.

#### A recap of when to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`

- `Rc<T>` enables multiple ownership. `Box<T>` and `RefCell<T>` have single owners.
- 
  - `Box<T>` allows immutable or mutable borrows checked at compile time.
  - `Rc<T>` only allows immutable borrows checked at compile time. 
  - `RefCell<T>` allows immutable or mutable borrows checked at runtime.
- Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

#### Interior Mutability continued

Aside, this won't compile, as we "cannot borrow immutable local variable 'x' as mutable":

```
fn main() {
    let x = 5;
    let y = &mut x;
}
```

It can be useful for a value to mutate itself in its methods but appear immutable to other code. `RefCell<T>` is one way to do this. 

```
pub trait Messenger {
    fn send(&self, msg: &str);
}

struct MockMessenger {
    sent_messages: vec<string>;
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.push(String::from(message));
    }
}

pub struct LimitTracker<'a, T: 'a + Messenger>  {
    messenger: &'a T,
    value: usize,
    max: usize,
}
```

In this case we want to use the `MockMessenger` instance to track the messages that `LimitTracker` has used it to send by calling `#self.messenger.send`. Unfortuneately, `LimitTracker` can't call a method on `MockMessenger` that causes it to mutate itself because `LimitTracker` only has an immutable reference to `MockMessager` (in the field `LimitTracker.messenger`).

This is also by design because we don't want a normal, non mock `Messenger` to mutate itself, which is why the method `#Messenger::send` signature specifies that it takes a immutable reference to a `Messenger`.

The way we fix this is by having `MockMessenger` hold a `RefCell<Vec<String>>`, which will allow for interior mutability:

```
use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: &str);
}

struct MockMessenger {
    sent_messages: RefCell<vec<string>>;
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

pub struct LimitTracker<'a, T: 'a + Messenger>  {
    messenger: &'a T,
    value: usize,
    max: usize,
}

#[test]
fn it_sends_an_over_75_percent_warning_message() {
    // --snip-- 

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
}
```

Notice that we use `#borrow_mut` to get a mutable reference to the `Vector` inside of `MockMessenger`, and we call `#borrow` to get an immutable reference when we want to test the content of the `Vector`. 

Also notice that if we had tried to use `Box<Vec<String>>` instead of `RefCell<Vec<String>>` the program would not have compiled because `#send` could not mutate the value inside of the `Box<Vec<String>>` without owning the `Box`, which it cannot because of its signature.

#### How it works under the hood.

When creating immutable and mutable references, we use `&` and `&mut`. When we have `RefCell<T>` we use `#borrow` and `#borrow_mut` methods to get `Ref<T>` and `RefMut<T>` smart pointers. Since both implement `Deref`, we can treat them like regular references.

`RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` pointers are active. Just like the compile time borrow checks, `RefCell<T>` guarantees at runtime that we can have many immutable borrows or one mutable borrow at any point in time. As such, this will panic at runtime: 

```
fn send(&self, message: &str) {
    let mut one_borrow = self.sent_messages.borrow_mut();
    let mut two_borrow = self.sent_messages.borrow_mut();
}
```

```
thread '...' panicked at 'already borrowed: BorrowMutError', ...
```

Similarily, if we tried to create a `Ref<T>` when a `RefMut<T>` is already in scope:

```
fn send(&self, message: &str) {
    let mut one_borrow = self.sent_messages.borrow_mut();
    let mut two_borrow = self.sent_messages.borrow(); // notice we are just asking for a immutable `Ref<T>`
}
```

```
thread '...' panicked at 'already mutably borrowed: BorrowError', ...
```

### Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>

`RefCell<T>` is commonly used with `Rc<T>`. Since `Rc<T>` allows you to have multiple owners of some data, but only gives immutable access to the data, you can put a `RefCell<T>` inside an `Rc<T>` to allow multiple owners to mutate some data. Similarily, you can put a `Rc<T>` inside a `RefCell<T>` to allow for a mutable reference to something owned by other owners.
Put simply:

`RefCell<Rc<T>>` - allows a reference to a value with multiple owners, which can be changed to reference another value.
`Rc<RefCell<T>>` - allows a variable to own a value, and change it, despite multiple owners.

Remember that the main advantage of `Rc<T>` is that by allowing for multiple owners, we don't have to assign a single owner at compile time, which can sometimes be difficult. 

See `chapter_15/src/mutable_list.rs` for an example of using a `Rc<RefCell<T>>` to create a Cons (linked) List with mutable values (but not mutable next pointers).

`Cell<T>` is similar to a `RefCell<T>` except instead of providing references to the inner value, the value is copied in and out of the `Cell<T>`

`Mutex<T>` provides interior mutability that is safe across threads.

**Challenge** Update the List in `chapter_15/src/mutable_list.rs` to allow for changing the "next" pointer in the Cons list by wrapping it in a `RefCell<T>` like we did the value of the Cons variant.

-> This is accomplished in `chapter_15/src/alternate_mutable_list.rs`.

### Reference Cylces Can Leak Memory

If we had a Cons list or similar recursive data structure that allowed for mutating the "next" pointer, we could easily create a directed acyclic graph with a reference cycle:

```
pub fn use_alternate_mutable_list() {
    println!("\nUsing alternate Mutable List \n");

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

We can see this cycle in the example in `chapter_15/src/cyclic_mutable_list.rs`. When we try to print our cyclic graph, the `Debug` formatter recurisively prints all the values within the struct. Since `a` contains `b`, when we print `#a.tail`, it prints the List `a` which includes the list `b`, so then it tries to print the list `b` which in turn includes the list `a`, and so on.

#### The Memory Leak

Assuming we don't call any recursive function that loops through the `List`s infinitely, A memory leak will occur at the end of the above function:

1. First the local variable `b` will go out of scope, and drop will be called on the reference bound to the local variable, so the reference count will drop to 1 (a.tail() still points to `Rc<List>` defined with `b`).
2. Second, the local variable `a` will go out of scope, and drop will be called on the reference bound to the local variable, so the reference count of the `Rc<List>` formerly pointed to by `a` will drop to 1 (The "tail" of the `Rc<List>` formerly referenced to by `b` still references `a`).

In the end each of the two `Rc<List>` had 1 reference stored on the stack, and one reference stored on the heap (by the other `Rc<List>`. Since both references stored on the stack have been cleared, both `Rc<List>`s are unreachable but have not been deallocated.

### Preventing Reference Cycles: Turning an Rc<T> into a Weak<T> 

We have seen that we can create a new reference to a `Rc<T>` by calling `Rc::clone`, which will increase the `strong_count` of the `Rc<T>` instance. The `Rc<T>` instance will only be cleaned up when its `strong_count` reaches zero.

We can also call `Rc::downgrade` on a `Rc<T>` to get a `Weak<T>` (weak reference) to the `Rc<T>` that will increase its `weak_count` rather than its `strong_count`. The `weak_count` does not need to be zero for the `Rc<T>` to be cleaned up.

Since weak references will not prevent an instance from being cleaned up, they can be used to prevent reference cycles. 

To use a `Weak<T>`, we have to first check if the value that it references has been dropped. We can call the `upgrade` method on a `Weak<T>` instance, which will return an `<Option<Rc<T>>>`. We will get a `Some<Rc<T>>` if the `Rc<T>` has _not_ been dropped, and a `None` if it has.

### Creating a Tree Data Structure

When we create a parent node with many children, and then drop the parent node, the child nodes should be dropped if there are not other references to them besides the one the parent had acquired. This suggests it would be appropriate for the parent node to have acquired a reference to its children using `#Rc::clone` to create a strong reference.

When we want a child node to reference its parent, however, we do not want the parent node to be dropped when its child is dropped. Therefore we will give a child node a `Weak<T>` reference to its parent with `#Rc::downgrade`.

**Challenge**: Explain how using a `strong_reference` to reference a parent from a child could cause the parent to be dropped. 

Answer: It only could if we didn't have another strong reference to the parent. This would be a bad usage of a tree: we shouldn't have only a reference to the middle of the tree.

## Chapter 16 (Fearless Concurrency)

A major goal of Rust is to leveral the ownership and type systems to provide concurrency errors at compile-time instead of at runtime.

This book talks about _concurrency_ when it means _concurrency and/or parallelism_ it considers _parallelism_ a subset of _concurrency_.

### Using Threads to Run Code Simultaneously

A _process_ and run independent parts of a program in _threads_

Certain problems arise because we can't guarantee the order in which parts of our code will run on different threads:

- _Race Conditions_: threads are accessing data or resources in an inconsistent order
- _Deadlocks_: two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing
- Bugs that happen only in certain situations and are hard to reproduce and fix reliably

_1:1_: One operating system thread per one language thread.

_green threads_: threads provided by the programming language executed in the context of a different number of operating system threads.

_M:N_: M green threads per N operating system threads (M and N may be different).

_runtime_: We're talking about code that is included by the language in every binary right now, not the _time that the code is ran_.

Rust has to take advantage of a small _runtime_ in order to call into C to maintain performance. Rust maintains a _1:1_ implementation of threading to help maintain its small _runtime_. _M:N_ models can be utilized via crates to take advantage of more control over which threads run when and lower costs of context switching.

#### Creating a New Thread with `spawn`

To create a new thread we pass a _closure_ to `thread::spawn`. There is no guarantee on the order in which threads run, and when the main (parent) thread shuts down, the spawned threads will end too.

In order to guarantee a thread gets run, and doesn't just end when the scope ends, we can save the return value of `thread::spawn` in a varaible. The type `thread::spawn` returns is a `JoinHandle`. A `JoinHandle` is an owned value that will wait for its thread to finish when we call `#join` on it.

Calling `#join` on the handle blocks the thread currently running until the thread represented by the handle terminates. _Blocking_ is when a thread is prevented from performing work or exiting.

### Using move Closures with Threads

In Chapter 13, we saw that closures can capture variables from their environment, Here we try to capture an immutable reference to a vector in a closure we are passing to a new thread: 

```
let v = vec![1,2,3];

let handle = thread::spawn(|| {
    println!("Here's a vector: {:?}", v);
});

handle.join().unwrap();
```

This will result in an error: 

```
25 |     let handle = thread::spawn(|| {
   |                                ^^ may outlive borrowed value `v`
26 |         println!("Here's a vector: {:?}", v);
   |                                           - `v` is borrowed here
   |
note: function requires argument type to outlive `'static`
  --> src/main.rs:25:18
   |
25 |       let handle = thread::spawn(|| {
   |  __________________^
26 | |         println!("Here's a vector: {:?}", v);
27 | |     });
   | |______^
```

The problem here is that the compiler can't tell how long the spawned thread will run, so it doesn't know whether the reference to `v` will always be valid. We're not allowed to run code like this because it could result in an invalid reference depending on what we do with the borrowed value:

```
let v = vec![1,2,3];

let handle = thread::spawn(|| {
    println!("Here's a vector: {:?}", v);
});

drop(v);

handle.join().unwrap();
```

The solution is to stop rust from inferring that we want the closure to borrow the captured variable and force the closure to take ownership of it:

```
let v = vec![1,2,3];

let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
});

handle.join().unwrap();
```

### Using Message Passing to Transfer Data Between Threads

_Message passing_ is when threads or actors communicate by sending each other messages containing data.

From the _Golang_ docs: "Do not communicate by sharing memory; instead, share memory by communicating.

_channel_: A programming concept used to send messages from a _transmitter_ to a _receiver_.

The _transmitter_ has methods we can call to send data.

The _receiver_ has methods we can call to check for messages.

A channel is _closed_ if either the transmitter or receiver half is dropped.

We can use `#std::sync::mpsc::channel` to create a _multiple producer single consumer_ channel. This will return a `(tx, rx)` (`Sender`, `Receiver`) tuple in the default case. 

We can use `#rx.recv` to block the current thread and return a `Result<T,E>` when a message is sent on the channel. Alternatively `#rx.try_recv` will return a `Result<T,E>` immediately. 

If we make our transmitter delay its message sending, the receiver will wait for the message to be sent with `#tx.recv`:

```
// multiple producer, single consumer
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("Hi");

    thread::sleep(Duration::from_millis(1000));
    tx.send(val).unwrap();
});

let received = rx.recv().unwrap();
println!("Got: {}", received);
```

If we send our message before telling the receiver to listen, the message will be waiting for the receiver when it decides to receive.

```
// multiple producer, single consumer
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("Hi");

    tx.send(val).unwrap();
});

thread::sleep(Duration::from_millis(1000));
let received = rx.recv().unwrap();
println!("Got: {}", received);
```

If we use `#try_recv` to check the channel for messages before there are any, we will receive an `Error`:

```
// multiple producer, single consumer
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("Hi");

    thread::sleep(Duration::from_millis(1000));
    tx.send(val).unwrap();
});

let received = rx.try_recv().unwrap();
println!("Got: {}", received);
```

The channel was empty when we tried to receive without blocking:
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Empty', src/main.rs:74:34
```

#### Receiving from closed channel: 

When a transmitter or receiver gets dropped, the channel is consdered closed. calling `rx.recv()` on when the receiver's channel is closed will result in an error:

```
fn use_channels() {
    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");

        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    let received = rx.recv().unwrap();
}
```

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError'
```

Or if using `rx.try_recv()`:

```
fn use_channels() {
    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");

        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    let received = rx.try_recv().unwrap();
}
```

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Disconnected'
```

#### Channels and Ownership Transference 

The signature of send shows that it takes ownership of whatever piece of data "t", it is sending:

```
pub fn send(&self, t: T) -> Result<(), SendError<T>>
```

As such, when we send a value, we cannot continue to use it in the transmitting thread.

Notice that `std::sync::mpsc::Receiver` implements Iterator:

```
for received in rx {
    println!("Got: {}", received);
}
```

Generate a new `std::sync::mpsc::Receiver` for a multiple producer pattern: `let tx1 = tx.clone()`.

### Shared State Concurrency

#### Using Mutexes to Allow Access to Data from One Thread at a Time

_Mutex_ (abbreviated forom "mutual exclusion") holds a lock, which  is a data structure that keeps track of who currenly has exclusive access to the data the _mutex_ holds.

#### Under the hood

A call to `m.lock` where m. is a `Mutex<T>` returns a _smart pointer_, _MutexGuard_. _MutexGuard_ implements _Deref_ to allow us a to acquire a reference to the inner data. _MutexGuard_'s implementation of _Drop_ automatically released the lock when the _MutexGuard_ goes out of scope. This prevents us from forgetting to release the _lock_. `Mutex<T>` also provides interior mutability like `RefCell<T>`, so you can pass an immutable reference to it and still mutate the value it holds.

Mutexes have two usage rules:

- You must attempt to acquire the lock before using the data.
- When you're done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

##### Acquiring the lock

The type system won't let us use the value held by the mutex before we acquire the lock.

To acquire the lock of the _mutex_ you call `m.lock()` which will block the current thread until the lock is available, then return a `Result<MutexGuard<T>, PoisonError<E>>`.

If a thread holding the lock panics, the call to `m.lock()` will fail, preventing from any other thread from acquiring the lock. 

##### Unlocking the data.

As stated above, since the _Mutex_ will return a _MutexGuard_ when we call `m.lock`, and `MutexGuard#drop` will automatically unlock the mutex when it goes out of scope. We can usually count on the _Mutex_ to be unlocked automatically.

#### Sharing Mutexes between threads

See examples `share_data_between_two_threads_with_mutexes` and `share_data_between_many_threads_with_mutexes` in `chapter_16/src/main.rs`.

We can't use `move` a mutex into closures going to two differnent threads, this will result in a compiler error: 

```
error[E0382]: use of moved value: `counter`
   --> src/main.rs:199:36
    |
195 |     let counter = Mutex::new(0);
    |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
199 |         let handle = thread::spawn(move || {
    |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
200 |             let mut num = counter.lock().unwrap();
    |                           ------- use occurs due to use in closure
```

The answer is to use multiple ownership with a _reference counting pointer_. We might try to use `Rc<Mutex<T>>`, but we would get another error: 

```
error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
   --> src/main.rs:173:18
    |
173 |       let handle = thread::spawn(move || {
    |  __________________^^^^^^^^^^^^^_-
    | |                  |
    | |                  `Rc<Mutex<i32>>` cannot be sent between threads safely
174 | |         let mut num = first_counter_clone.lock().unwrap();
175 | |
176 | |         *num += 1;
177 | |     });
    | |_____- within this `[closure@src/main.rs:173:32: 177:6]`
    |
    = help: within `[closure@src/main.rs:173:32: 177:6]`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`
    = note: required because it appears within the type `[closure@src/main.rs:173:32: 177:6]`
note: required by a bound in `spawn`
   --> /Users/carsonrajcan/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/mod.rs:625:8
    |
625 |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`
```

It turns out `Rc<T>` does not use any "_concurrency primitives_" to make sure that changes to the reference count can't be interrupted by another thread. We need to use `Arc<T>` instead.

`Arc<T>` is the _atomically reference counted_ type. It is safe to use in concurrent situations. _Atomics_ are a "_concurrency primitive_", basically they work just like primitive types but they are thread-safe, this safety comes with a performance penalty, so we don't use them in single threaded scenarios. More about _Atomics_ can be found in `std::sync::atomic`. 

`Arc<T>` has the same api as `Rc<T>`, so we can call `Arc::new(Mutex::new(0))` and `Arc::clone(&counter)`

##### Deadlocks

_deadlocks_ occur when an operation needs to lock two resources and two threads have acquired one of the locks, causing them to wait for each other forever. A program with 2 or more threads, and two or more `Mutex<T>`s could fall victim to _deadlocks_.

**Challenge** Create a program with _deadlocks_, then research _deadlock_ mitigation strategies for mutexes to fix the program. Refer to the docs for `Mutex<T>` and `MutexGuard` for useful information.

### Extensible Concurrency with the Sync and Send Traits

Most concurrency features come from the standard library, not the core Rust language. This is evident by all the imports made in `chapter_16/src/main.rs`. Two concurrency concepts are enbedded in the language, `std::marker` traits `Sync` and `Send`

`std::marker` is a module of primitive traits and types representing basic properties of types.

#### Allowing Transference of Ownership Between Threads with Send

Types that implement `Send` can be transferred between threads. `Rc<T>` is not `Send`, because two or more threads could try to update the reference count at the same time. Any type composed entirely of `Send` types is automaticaly marked `Send`. 

#### Allowing Access from Multiple Threads with Sync

Types that implement `Sync` can be referenced from multiple threads.

_Any type `T` is `Sync` if `&T` is `Send`_

That is, the reference of any `Sync` type can be sent safely to another thread.

Since `Rc<T>` is not `Send`, it is also not `Sync`.

`RefCell<T>` and the other `Cell<T>` types are not `Sync`. The implementation of borow checking `RefCell<T>` does at runtime is not thread-safe.

Remember, `Cell<T>` is similar to a `RefCell<T>` except instead of providing references to the inner value, the value is copied in and out of the `Cell<T>`

Since `Mutex<T>` is `Sync`, we can use it instead of `RefCell<T>` or other `Cell<T>` to share access across multiple threads.

**Question** Why is the runtime borrow checking implemented on `RefCell<T>` not thread-safe?

Possible Answer: It has to enforce the rules about the number of immutable and mutable references in existence but has not way to know what references exist in other threads.

#### Implementing `Send` and `Sync` Manually is `Unsafe`

`Send` and `Sync` are derived automatically for types that are made up of only `Send` and `Sync` types. Also, since they are `std::marker` traits, they don't have any methods to implement.

To implement these traits manually, we would have to implement `unsafe` Rust code. Doing so should be done with great care. The [Rustonomicon](https://doc.rust-lang.org/stable/nomicon/) has more information about these guarantees and how to uphold them.

#### Paraphrasing in Summary:

Reference counting requires ownership and we have to be sure about what threads are doing to change reference counts (`Rc<T>` is not `Send`, can't be sent/owned among multiple theads)

Checking borrow rules requires that we are sure about what other threads are doing with their references. (`RefCell<T>` is not `Sync`, can't be shared among threads)

`Rc<T>` is not `Send`, so it's not `Sync` either.

For sharing data by message passing: use `std::sync::mpsc#channel` with std::sync::mpsc::{Sender, Receiver}.

For sharing ownership of data, use `std::sync::Arc<std::sync::Mutex<T>>`.

## Chapter 17 (Object-Oriented Programming Features of Rust)

_Encapsulation_: The implementation details of an object aren't accessible to the code using that object. The only way to interact with an object is through its public API.

### Inheritance as a Type System and as Code Sharing

Rust does not have inheritance.

Usually you choose to use inheritance for 2 reasons:

1. To reuse code. You can share Rust code using default trait method implementations instead of using inheritance. You can even make a trait with only default trait implementations:

```
trait Summary {
    fn summarize(&self) -> String {
        String::from("foo")
    }
}

struct Foo {
    biz: i32,
}

impl Summary for Foo {}
```

You can also override default implementations by redefining them for your type.

If you wanted to match the pattern of calling `super` then performing some other work in your override function, you could use a conditional implementation where you impl the trait for types that implement some other trait. That way you could call methods from that other trait in your override method

2. The other reason to use inheritance is to allow a child type to be used in the same places as the parent type. We usually say that code that case work with data of multiple types is _polymorphic_. Rust uses generics to abscrat over different possible types and trait bounds to impose constraints on what those types must provide. This is referred to as _bounded parametric polymorphism_.

Rust uses _trait objects_ instead of inheritance to avoid two common inheritance problems:

1. Sharing too much code.
2. Only being able to inherit from one parent 

## Using Trait Objects that Allow for Values of Different Types.

In Chapter 8 we defined an enum `SpreadsheetCell` to hold *integer*, *float* and *text* variants in the same `Vec`. We did not know which enum variants would be in which cells in the `Vec` at compile time, but because knew we had a fixed set of `SpreadSheetCell` types, we were able to use an `enum` to get the code to compile.

If we wanted our library user to be able to extend the set of types they can use with our API, an enum will not work, because it cannot be extended without modifying the API.

We need to use _trait objects_. A _trait object_ points to an instance of a type that implements the trait we specify. We create a _trait object_ by specifying a pointer `&` or `Box<T>` and specifying the relevant trait. Rust can ensure at compile time that any value used in a trait object will implmement the trait object's trait.

_Trait objects_ are more like OOP objects because they combine data and behavior more so than our Rust types usually do. They do not, however, allow for adding data to an object through inheritance.

Definine a _trait object_

```
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```
_trait objects_ are not allways collection types:

```
pub struct MyScreen {
    pub components: Box<dyn Draw>,
}
```

Notice that we don't need to define generic type parameters. Generic type parameters guarantee that we only substitute one concrete type at a time, for each paramter. So everywhere you see `T` we're talking about the same concrete type. With _trait objects_ we're allowed to have multiple concrete types fill in for the object, so we don't need to specify a generic type paramter for the implementation block or the method. 

When you know a collection is going to be homogoneous, you should prefer to use generics and trait bounds to allow for monomorphization, which is the compiler's behavior of filling in concrete types at compile time. With _trait objects_, the compiler doesn't know what types will be filled in, so monomorphization is impossible.

The idea of being concerned with the messages a value responds to rather than the value's concrete type is similar to _duck typing_. Our `Screen#run` method in `chapter_17/src/lib.rs` leverages _duck typing_ in that it doesn't care what each component's type is, or how it implements `Draw`. The advantage Rust has over most languages that implement _duck typing_ is that it still will not compile if it is possible for a message (method) to be send (called) on an instance of a type that doesn't implement it.

```
let screen = Screen {
    components: vec![
        Box::new(String::From("Hi")),
    ],
};

screen.run();
```


```
error[E0277]: the trait bound `String: Draw` is not satisfied
  --> src/main.rs:33:13
   |
33 |             Box::new("hello".to_string()),
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
   |
   = note: required for the cast to the object type `dyn Draw`
```

Here, `String` does not implement the _trait objects_'s trait.

### Trait Objects Perform Dynamic Dispatch

_static dispatch_: when the compiler knows at comiple time what code you are calling.
_dynamic dispatch_: when the compiler can't tell at compile time which method you're calling. The compiled code will determine what method to call at runtime.

While _trait objects_ allow us some flexibibility over using generics, there is a runtime cost. The running program must spend time finding the right method to call based on the type of the value enclosed in the _trait_object_.

### Object Safety is Required for Trait Objects

Only _object-safe_ traits can be made into trait objects. A trait is _object-safe_ if all the methods defined in the trait have the following properties:

- the return type isn't Self
- There are no generic type parameters

Once you have used a _trait object_, rust no longer knows the concret type that's implementing that trait.

trait objects do not rememver the concrete type of the values that they hold. So you wouldn't be able to call a trait method that returns `Self` on a trait object, because it would not know what type to return. Similarily, a trait method with a generic type parameter could not be called by a trait object, because it would not know what concrete type to fill in.

Suppose we tried to make a trait object for `Clone` which has a trait method that returns an instance of `Self`:

```
pub trait Clone {
    fn clone(&self) -> Self;
}

pub struct Screen {
    pub components: Vec<Box<Clone>>,
}
```

We get an error:

```
error[E0038]: the trait `Clone` cannot be made into an object
 --> src/lib.rs:6:29
  |
6 |     pub components: Vec<Box<dyn Clone>>,
  |                             ^^^^^^^^^ `Clone` cannot be made into an object
  |
  = note: the trait cannot be made into an object because it requires `Self: Sized`
  = note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
```

Here we see that a "vtable" cannot be made for a trait object using `Clone` because the return type of `Clone::clone` cannot be resolved dynamically, since the trait object does not know anything about the concrete types it holds.

### Implementing an Object-Oriented Design Pattern

In addition to being able to hold values of arbitrary type, trait objects have the advantage over enums in that each type your trait object can hold can have its own implementation. That is, each type that implements the trait is responsible for its own implementation. With enums, the variants cannot have their own implementation, they can only hold types that have their own implementation. 

Because trait objects allow us to hold any type that implements a certain trait, they are perfect for implementing a _state pattern_. We can use a trait object to hold _state objects_, which are values that can represent a certain state, and implement behavior that is related to that state.

An advantage of the _state pattern_ is that it allows for a separation of concerns. Any data structure holding a state object does not need to know the rules of how to manipulate (mutate) the state. That logic can be encoded in the _state objects_ themselves, so if the rules around how the state can be mutated change, the data structures that hold the _state object_ do not need to change, they can just continuing passing messages to the state objects.

#### The `#take` Method

The `#take` method on `Option<T>` returns the enclosed `Some<val>` value and leaves a `None` value in its place. 

This is useful when there is a reference to something holding an `Option<T>` and we want to transfer ownership of the enclosed `Some<T>` value. By replacing the `Some<val>` with `None`, we are able to acquire ownership of `val` without violating the borrowing rules that guarantee the reference will remain valid.

An example is in `chapter_17/src/post.rs` in `Post#request_review`. 

```
pub fn request_review(&mut self) {
    if let Some(s) = self.state.take() {
        self.state = Some(s.request_review())
    }
}
```

Since the signature of `State#request_review()` requires ownership of self: 

```
fn request_review(self: Box<Self>) -> Box<dyn State> {
```

It is required that we take ownership of the _state object_ held by the `Post.state` field. This will also force the old _state object_ to be dropped (and freed) when it creates a new state object for the new state.

##### How to remember to use this.

When you have a mutable reference to an `Option<T>` which references something else (on the heap usually), and you want to replace what it is pointing to but can't invalidate the reference.

#### The `#as_ref` Method

`#as_ref` is useful when we want a reference to the value inside an `Option<T>` rather than ownership of the value. Similar to the usecase for `#take`, we would use `#as_ref` when we only have an immutable reference to an `Option<T>`.

```
pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(&self);
}
```

Here we want to call a method on the _state object_ held in an `Option<T>` inside of `Post.state`. We don't have ownership of the `Post` (&self), so we can't take ownership of the _state object_. We also don't have a mutable reference, so we can't use `#take` to take ownership of the _state object_ by replacing it temporarily with `None`. Our only option is to get an immutable reference to the internal _state object_.

Note that this is our only option because calling `unwrap()` on the option without borrowing the option's content first will always move the content, even if we don't assign the option's content.

```
pub fn content(&self) -> &str {
    let r = &self.state.unwrap();
    r.content(&self)
}
```

```
error[E0507]: cannot move out of `self.state` which is behind a shared reference
  --> src/post.rs:19:18
   |
19 |         let r = &self.state.unwrap();
   |                  ^^^^^^^^^^ move occurs because `self.state` has type `Option<Box<dyn State>>`, which does not implement the `Copy` trait
   |
help: consider borrowing the `Option`'s content
   |
19 |         let r = &self.state.as_ref().unwrap();
   |                            +++++++++
```

**TODO**: Practice ownerships/borrowing rules with `Option<T>`, `#unwrap`, `#take`, and `#as_ref`

In the end we avoid having to match against an enum everywhere we want to use a `Post` in a certain state. However, to guarantee _object-saftey_ we must also duplicate any methods that return `Self` in our trait objects.