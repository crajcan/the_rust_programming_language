# Chapter 1

```
run clippy: cargo clippy --fix --allow-staged || --allow-dirty
run rustfmt: rustfmt src/main.rs || src.lib.rs
```

# Chapter 2

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

# Chapter 3

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

# Chapter 4

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

# Chapter 5

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

# Chapter 6 (Enums and Pattern Matching)

Chapter 6 was pretty straight forward. As a challenge:

Write a method that takes in a mutable reference, then based on a match statement, conditionally passes the mutable reference to another method that mutates it. (Completed in #double_if_exists)

# Chapter 7 (Module stuff I mostly knew)

# Chapter 8 (Array and collection stuff I knew)

# Chapter 9 (Panics and Error Handling)

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