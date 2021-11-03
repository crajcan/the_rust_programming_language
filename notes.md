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
````
`expect` is implemented specifically for std::io::Result and takes in a std::io::Result. It panics if that Result is an instance of Err, or unpacks return value if the Result is an instance of Ok.

`cargo update` will ignore versions pinned in `Cargo.lock` and update to the dependency to the latest version that fits the api specification of the version pinned in Cargo.toml. It will not make major version updates.

`extern crate rand;` - Lets the compiler know we'll be using rand as an externa dependency.

`use rand::Rng;` - brings the Rng trait into scope, which defines random number generation methods on instances of `rand`.
