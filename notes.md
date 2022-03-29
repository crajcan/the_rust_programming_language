# Notes From The Text<!-- omit in toc -->

These are notes that I took every time I encountered a fact I didn't already know, or was introduced to
a concept that I didn't immediately understand

The notes that follow were created whenever I encountered a fact/concept that was:

Unknown to me
Non-trivial
Important
Hard to remember


## Table of Contents<!-- omit in toc -->
- [Chapter 1](#chapter-1)
- [Chapter 2](#chapter-2)
- [Chapter 3](#chapter-3)
  - [Arrays](#arrays)
- [Chapter 4](#chapter-4)
  - [String](#string)
    - [Move](#move)
    - [Copy types](#copy-types)
    - [Ownership with functions](#ownership-with-functions)
    - [Borrowing & References](#borrowing--references)
    - [Mutable References](#mutable-references)
    - [Danginling references](#danginling-references)
    - [Recap of Reference Rules:](#recap-of-reference-rules)
  - [String Slice](#string-slice)
- [Chapter 5](#chapter-5)
  - [Structs](#structs)
    - [Printing Structs](#printing-structs)
    - [Automatic Referencing and Dereferencing](#automatic-referencing-and-dereferencing)
    - [Timing of creating references and mutable references](#timing-of-creating-references-and-mutable-references)
- [Chapter 6 (Enums and Pattern Matching)](#chapter-6-enums-and-pattern-matching)
- [Chapter 7 (Module stuff I mostly knew)](#chapter-7-module-stuff-i-mostly-knew)
- [Chapter 8 (Array and collection stuff I knew)](#chapter-8-array-and-collection-stuff-i-knew)
- [Chapter 9 (Panics and Error Handling)](#chapter-9-panics-and-error-handling)
    - [match vs ?](#match-vs-)
    - [Wrapping types for consiseness:](#wrapping-types-for-consiseness)
    - [Definition of Result:](#definition-of-result)
- [Chapter 10 (Generics, Traits, and Lifetimes)](#chapter-10-generics-traits-and-lifetimes)
  - [Performance](#performance)
  - [Orphan rule](#orphan-rule)
  - [Default implmentations](#default-implmentations)
  - [Trait Bounds](#trait-bounds)
    - [Copy And Clone](#copy-and-clone)
  - [Conditional Implmentations](#conditional-implmentations)
    - [Blanket Implementations](#blanket-implementations)
    - [Combining the two](#combining-the-two)
  - [Lifetimes](#lifetimes)
    - [Dangling references](#dangling-references)
    - [Generic Lifetimes in Functions](#generic-lifetimes-in-functions)
  - [Lifetime Annotations in Struct Definitions](#lifetime-annotations-in-struct-definitions)
  - [Lifetime Elision](#lifetime-elision)
    - [Input Lifetimes](#input-lifetimes)
    - [Output Lifetimes](#output-lifetimes)
    - [The Compiler's 3 lifetime inference rules](#the-compilers-3-lifetime-inference-rules)
    - [Lifetime Annotations in Method Definitions](#lifetime-annotations-in-method-definitions)
  - [The Static Lifetime](#the-static-lifetime)
  - [Syntax for Generic type parameters, Trait Bounds and Lifetimes all together](#syntax-for-generic-type-parameters-trait-bounds-and-lifetimes-all-together)
- [Chapter 11 (Writing Automated Tests)](#chapter-11-writing-automated-tests)
  - [Doc Tests](#doc-tests)
  - [Custom Failure Messages](#custom-failure-messages)
  - [Should panic](#should-panic)
  - [Testing options](#testing-options)
    - [Specifying parallelism](#specifying-parallelism)
    - [Display print output from passing tests](#display-print-output-from-passing-tests)
    - [Ignoring specific tests unless requested](#ignoring-specific-tests-unless-requested)
  - [Integration tests](#integration-tests)
    - [Running a specific integration test file](#running-a-specific-integration-test-file)
    - [Helper code for integration tests](#helper-code-for-integration-tests)
- [Chapter 12 (An I/O project: Building a Command Line Program)](#chapter-12-an-io-project-building-a-command-line-program)
  - [Reading Argument Values](#reading-argument-values)
    - [Side note on importing modules](#side-note-on-importing-modules)
    - [Annotating types to create vectors](#annotating-types-to-create-vectors)
  - [Separation of Concerns for Binary Projects](#separation-of-concerns-for-binary-projects)
  - [Idiomatic Rust](#idiomatic-rust)
    - [Extracting the Command Line Arg Parsing Logic](#extracting-the-command-line-arg-parsing-logic)
      - [Summary](#summary)
    - [Extracting Logic From Main.](#extracting-logic-from-main)
      - [Summary](#summary-1)
    - [Splitting Code into a Library Crate](#splitting-code-into-a-library-crate)
    - [Using Test-Driven Development](#using-test-driven-development)
    - [Summary](#summary-2)
    - [Adding a case-insenstive search function](#adding-a-case-insenstive-search-function)
    - [Summary](#summary-3)
    - [Adding a environment variable for case-insenstive search](#adding-a-environment-variable-for-case-insenstive-search)
    - [Summary](#summary-4)
    - [Printing error messages to the stderr and successful output to stdout](#printing-error-messages-to-the-stderr-and-successful-output-to-stdout)
    - [#unwrap_or_else](#unwrap_or_else)
    - [Trait Objects](#trait-objects)
- [Chapter 13 (Iterators and Closures)](#chapter-13-iterators-and-closures)
    - [Fn traits](#fn-traits)
    - [Capturing the environment with closures](#capturing-the-environment-with-closures)
  - [Iterators](#iterators)
    - [A look under the hood](#a-look-under-the-hood)
      - [next](#next)
  - [Custom Iterators](#custom-iterators)
  - [Zip](#zip)
  - [Upgrading our minigrep program from Chapter 12](#upgrading-our-minigrep-program-from-chapter-12)
  - [Upgrading #search](#upgrading-search)
    - [Summary](#summary-5)
  - [Zero-Cost Abstractions](#zero-cost-abstractions)
  - [Unrolling](#unrolling)
- [Chapter 14 (More about Cargo and Crates.io)](#chapter-14-more-about-cargo-and-cratesio)
  - [Release Profiles](#release-profiles)
  - [Overriding a Release Setting:](#overriding-a-release-setting)
  - [Publishing a Crate to Crates.io](#publishing-a-crate-to-cratesio)
    - [Documentation Comments](#documentation-comments)
    - [Documenation Comments as Tests](#documenation-comments-as-tests)
    - [Commenting Contained Items](#commenting-contained-items)
  - [Re-Exporting with `pub use`](#re-exporting-with-pub-use)
  - [Publishing a Crate](#publishing-a-crate)
  - [Cargo Workspaces](#cargo-workspaces)
    - [Depending on an External Crate in a Workspace](#depending-on-an-external-crate-in-a-workspace)
    - [Testing within a workspace](#testing-within-a-workspace)
    - [Publishing crates from a workspace](#publishing-crates-from-a-workspace)
  - [Cargo Install](#cargo-install)
  - [Extending Cargo with Custom Commands](#extending-cargo-with-custom-commands)
- [Chapter 15 (Smart Pointers)](#chapter-15-smart-pointers)
  - [Implementation of Smart Pointers](#implementation-of-smart-pointers)
    - [Common Smart Pointer Types](#common-smart-pointer-types)
  - [Using `Box<T>` to Point to Data on the Heap](#using-boxt-to-point-to-data-on-the-heap)
  - [Recursive Data Types](#recursive-data-types)
    - [Determining the Space Needed to Store a Non-Recursive Data Type](#determining-the-space-needed-to-store-a-non-recursive-data-type)
    - [Back to Recursive Types](#back-to-recursive-types)
  - [Treating Smart Pointers Like Regular References with the Deref Trait](#treating-smart-pointers-like-regular-references-with-the-deref-trait)
    - [Box<T> vs &T](#boxt-vs-t)
    - [Implementing our own Smart Pointer.](#implementing-our-own-smart-pointer)
    - [Deref Coercion In Action:](#deref-coercion-in-action)
  - [Deref Coercion and Mutability](#deref-coercion-and-mutability)
  - [The `Drop` Trait](#the-drop-trait)
    - [Dropping a value early.](#dropping-a-value-early)
    - [`#std::mem::drop`](#stdmemdrop)
  - [`Rc<T>`, the Reference Counting Smart Pointer](#rct-the-reference-counting-smart-pointer)
    - [Trying to Implement Multiple Ownership with `Box<T>`](#trying-to-implement-multiple-ownership-with-boxt)
    - [Implementing Multiple Ownership with `Rc<T>`](#implementing-multiple-ownership-with-rct)
  - [Interior Mutability](#interior-mutability)
    - [Enforcing borrowing Rules at Runtime with `RefCell<T>`](#enforcing-borrowing-rules-at-runtime-with-refcellt)
    - [A recap of when to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`](#a-recap-of-when-to-choose-boxt-rct-or-refcellt)
    - [Interior Mutability continued](#interior-mutability-continued)
    - [How it works under the hood.](#how-it-works-under-the-hood)
  - [Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>](#having-multiple-owners-of-mutable-data-by-combining-rct-and-refcellt)
  - [Reference Cylces Can Leak Memory](#reference-cylces-can-leak-memory)
    - [The Memory Leak](#the-memory-leak)
  - [Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>](#preventing-reference-cycles-turning-an-rct-into-a-weakt)
  - [Creating a Tree Data Structure](#creating-a-tree-data-structure)
- [Chapter 16 (Fearless Concurrency)](#chapter-16-fearless-concurrency)
  - [Using Threads to Run Code Simultaneously](#using-threads-to-run-code-simultaneously)
    - [Creating a New Thread with `spawn`](#creating-a-new-thread-with-spawn)
  - [Using move Closures with Threads](#using-move-closures-with-threads)
  - [Using Message Passing to Transfer Data Between Threads](#using-message-passing-to-transfer-data-between-threads)
    - [Receiving from closed channel:](#receiving-from-closed-channel)
    - [Channels and Ownership Transference](#channels-and-ownership-transference)
      - [Iterating over received messages](#iterating-over-received-messages)
      - [Using multiple producers](#using-multiple-producers)
  - [Shared State Concurrency](#shared-state-concurrency)
    - [Using Mutexes to Allow Access to Data from One Thread at a Time](#using-mutexes-to-allow-access-to-data-from-one-thread-at-a-time)
    - [Under the hood](#under-the-hood)
      - [Acquiring the lock](#acquiring-the-lock)
      - [Unlocking the data.](#unlocking-the-data)
    - [Sharing Mutexes between threads](#sharing-mutexes-between-threads)
      - [Deadlocks](#deadlocks)
  - [Extensible Concurrency with the Sync and Send Traits](#extensible-concurrency-with-the-sync-and-send-traits)
    - [Allowing Transference of Ownership Between Threads with Send](#allowing-transference-of-ownership-between-threads-with-send)
    - [Allowing Access from Multiple Threads with Sync](#allowing-access-from-multiple-threads-with-sync)
    - [Implementing `Send` and `Sync` Manually is `Unsafe`](#implementing-send-and-sync-manually-is-unsafe)
    - [Paraphrasing in Summary:](#paraphrasing-in-summary)
- [Chapter 17 (Object-Oriented Programming Features of Rust)](#chapter-17-object-oriented-programming-features-of-rust)
  - [Inheritance as a Type System and as Code Sharing](#inheritance-as-a-type-system-and-as-code-sharing)
- [Using Trait Objects that Allow for Values of Different Types.](#using-trait-objects-that-allow-for-values-of-different-types)
  - [Trait Objects Perform Dynamic Dispatch](#trait-objects-perform-dynamic-dispatch)
  - [Object Safety is Required for Trait Objects](#object-safety-is-required-for-trait-objects)
  - [Implementing an Object-Oriented Design Pattern](#implementing-an-object-oriented-design-pattern)
    - [The `#take` Method](#the-take-method)
      - [How to remember to use this.](#how-to-remember-to-use-this)
    - [The `#as_ref` Method](#the-as_ref-method)
- [Chapter 18 (Patterns and Matching)](#chapter-18-patterns-and-matching)
  - [While let](#while-let)
  - [let statements](#let-statements)
  - [Multiple Assignment](#multiple-assignment)
  - [Refutability](#refutability)
  - [Pattern Syntax](#pattern-syntax)
    - [Matching literals:](#matching-literals)
    - [Matching named variables:](#matching-named-variables)
    - [Multiple Patterns in One Match Arm](#multiple-patterns-in-one-match-arm)
    - [Matching Ranges](#matching-ranges)
    - [Destructuring](#destructuring)
      - [Structs](#structs-1)
      - [Enums](#enums)
    - [Ignoring Entire Values With _](#ignoring-entire-values-with-_)
    - [Ignoring Parts of a Value with a Nested _](#ignoring-parts-of-a-value-with-a-nested-_)
    - [Ignoring usused variables with a leading _](#ignoring-usused-variables-with-a-leading-_)
    - [Ignoring Remaining Parts of a Value with ..](#ignoring-remaining-parts-of-a-value-with-)
      - [Using .. in patterns must be unambiguous:](#using--in-patterns-must-be-unambiguous)
  - [Creating References in Patterns with ref and ref mut](#creating-references-in-patterns-with-ref-and-ref-mut)
    - [Creating an Immutable Reference While Destructuring.](#creating-an-immutable-reference-while-destructuring)
    - [Creating a mutable reference while restructuring.](#creating-a-mutable-reference-while-restructuring)
  - [Extra Conditionals in Match Guards](#extra-conditionals-in-match-guards)
    - [| with match guards](#-with-match-guards)
  - [@ Bindings](#-bindings)
  - [Match Ergonomics:](#match-ergonomics)
- [Chapter 19 (Advanced Features)](#chapter-19-advanced-features)
  - [Unsafe Rust](#unsafe-rust)
    - [Unsafe Superpowers](#unsafe-superpowers)
    - [Dereferencing a Raw Pointer](#dereferencing-a-raw-pointer)
    - [Creating a Safe Abstraction over Unsafe Code](#creating-a-safe-abstraction-over-unsafe-code)
    - [Using `extern` Functions to Call External Code](#using-extern-functions-to-call-external-code)
    - [Calling Rust Functions From Other Languages](#calling-rust-functions-from-other-languages)
    - [Accessing or Modifying a Mutable Stack Variable](#accessing-or-modifying-a-mutable-stack-variable)
    - [Implementing an Unsafe Trait.](#implementing-an-unsafe-trait)
  - [Advanced Lifetimes](#advanced-lifetimes)
    - [Ensuring One Lifetime Outlives Another with Lifetime Subtyping](#ensuring-one-lifetime-outlives-another-with-lifetime-subtyping)
    - [Lifetime Bounds on References to Generic Types](#lifetime-bounds-on-references-to-generic-types)
    - [Inference of Trait Object Lifetimes](#inference-of-trait-object-lifetimes)
  - [Advanced Traits](#advanced-traits)
    - [Specifying Placeholder Types in Trait Definitions with Associated Types](#specifying-placeholder-types-in-trait-definitions-with-associated-types)
    - [Default Generic Type Parameters and Operator Overloading](#default-generic-type-parameters-and-operator-overloading)
    - [Operator Overloading](#operator-overloading)
    - [Calling Multiple Methods From One Type With The Same Name](#calling-multiple-methods-from-one-type-with-the-same-name)
    - [One Type Two Traits with Overlapping Associated Function Names](#one-type-two-traits-with-overlapping-associated-function-names)
    - [Using Supertraits to Require One Trait's Functionality Within Another Trait](#using-supertraits-to-require-one-traits-functionality-within-another-trait)
    - [Using the NewType Pattern to Implement External Traits on External Types](#using-the-newtype-pattern-to-implement-external-traits-on-external-types)
  - [Advanced Types](#advanced-types)
    - [Using the NewType Pattern for Type Safety and Abstraction](#using-the-newtype-pattern-for-type-safety-and-abstraction)
    - [Creating Type Synonyms with Type Aliases](#creating-type-synonyms-with-type-aliases)
    - [The Never Type that Never Returns](#the-never-type-that-never-returns)
    - [Dynamically Sized Types and the Sized Trait](#dynamically-sized-types-and-the-sized-trait)
  - [Advanced Functions and Pointers](#advanced-functions-and-pointers)
    - [Function Pointers](#function-pointers)
    - [Returning Closures](#returning-closures)
- [Chapter 20 (Final Project: Building a Multithreaded Web Server)](#chapter-20-final-project-building-a-multithreaded-web-server)
  - [Building a Single Threaded Web Server](#building-a-single-threaded-web-server)
    - [Listening for a Connection](#listening-for-a-connection)
    - [Reading the Request](#reading-the-request)
    - [A Closer Look at an HTTP Request](#a-closer-look-at-an-http-request)
    - [Writing a Response](#writing-a-response)
    - [Validating the Request and Selectively Responding](#validating-the-request-and-selectively-responding)
  - [Turning our Single-Threaded Server into a Multi-threaded server](#turning-our-single-threaded-server-into-a-multi-threaded-server)
    - [Improving Throughput with a Thread Pool](#improving-throughput-with-a-thread-pool)
    - [Compiler Driven Development](#compiler-driven-development)
    - [Making Each Worker's Thread Execute Jobs.](#making-each-workers-thread-execute-jobs)
  - [Graceful Shutdown and Cleanup](#graceful-shutdown-and-cleanup)
    - [Implementing the Drop Trait on `ThreadPool`](#implementing-the-drop-trait-on-threadpool)
    - [Signaling to the Threads to Stop Listening for Jobs](#signaling-to-the-threads-to-stop-listening-for-jobs)
  - [Creating an AsyncIO server.](#creating-an-asyncio-server)
    - [Adding an async runtime](#adding-an-async-runtime)
- [Apendix A: Keywords](#apendix-a-keywords)
- [Appendix B: Operators and Symbols](#appendix-b-operators-and-symbols)
  - [Operators](#operators)
  - [Symbols](#symbols)
    - [Symbols that appear on their own](#symbols-that-appear-on-their-own)
    - [Symbols that appear in the context of a path](#symbols-that-appear-in-the-context-of-a-path)
    - [Symbols that appear in the context of generic type paramters](#symbols-that-appear-in-the-context-of-generic-type-paramters)
    - [Symbols that appear in the context of constraining generic type parameters with trait bounds](#symbols-that-appear-in-the-context-of-constraining-generic-type-parameters-with-trait-bounds)
    - [Symbols that appear in the context of calling or defining macros and specifying attributes on an item](#symbols-that-appear-in-the-context-of-calling-or-defining-macros-and-specifying-attributes-on-an-item)
    - [Symbols that create comments](#symbols-that-create-comments)
    - [Symbols that appear in the context of using tuples](#symbols-that-appear-in-the-context-of-using-tuples)
    - [Square Brackets](#square-brackets)
- [Appendix C: Derivable Traits](#appendix-c-derivable-traits)
  - [Partial Eq and Eq](#partial-eq-and-eq)
  - [PartialOrd and Ord](#partialord-and-ord)
  - [Clone and Copy for Duplicating Values](#clone-and-copy-for-duplicating-values)
  - [Hash for Mapping a Value to a Value of Fixed Size](#hash-for-mapping-a-value-to-a-value-of-fixed-size)
  - [`Default` for Default Values](#default-for-default-values)
  - [New Version Appendix D - Useful Development Tools](#new-version-appendix-d---useful-development-tools)
    - [Automatic Formatting with `rustfmt`](#automatic-formatting-with-rustfmt)
    - [Fix Your Code with `rustfix`](#fix-your-code-with-rustfix)
    - [More Lints with Clippy](#more-lints-with-clippy)
  - [Appendix D Macros](#appendix-d-macros)
    - [The 4 types of Macros in Rust](#the-4-types-of-macros-in-rust)
    - [Declarative Macros with `macro_rules!`](#declarative-macros-with-macro_rules)
      - [An example: `vec!`](#an-example-vec)
      - [Macro Pattern Syntax](#macro-pattern-syntax)
    - [Pattern within the match arm](#pattern-within-the-match-arm)
  - [Procedural Code for Generating Code from Attributes](#procedural-code-for-generating-code-from-attributes)
  - [Writing a Custom `derive` macro](#writing-a-custom-derive-macro)
    - [Declaring our `hello_macro_derive` crate to be a procedural macro crate](#declaring-our-hello_macro_derive-crate-to-be-a-procedural-macro-crate)
  - [Attribute-like Macros](#attribute-like-macros)
  - [Function-like Macros](#function-like-macros)
  - [(Unrelated: Referring to type at runtime)](#unrelated-referring-to-type-at-runtime)

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

Conditionally implement a trait for a generic type whenever the generic type's enclosed type implements some trait:

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

## Chapter 13 (Iterators and Closures)

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

A _process_ can run independent parts of a program in _threads_

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

When a transmitter or receiver gets dropped, the channel is considered closed. calling `rx.recv()` on when the receiver's channel is closed will result in an error:

```
fn use_channels() {
    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");

        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();

    // allow the child thread time to be cleaned up
    thread::sleep(Duration::from_millis(100));

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

    // allow the child thread time to be cleaned up
    thread::sleep(Duration::from_millis(500));

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

##### Iterating over received messages

Notice that `std::sync::mpsc::Receiver` implements Iterator:

```
for received in rx {
    println!("Got: {}", received);
}
```

##### Using multiple producers

Generate a new `std::sync::mpsc::Producer` for a multiple producer pattern: `let tx1 = tx.clone()`.

### Shared State Concurrency

#### Using Mutexes to Allow Access to Data from One Thread at a Time

_Mutex_ (abbreviated from "mutual exclusion") holds a lock, which is a data structure that keeps track of who currenly has exclusive access to the data the _mutex_ holds.

#### Under the hood

A call to `m.lock` where `m`. is a `Mutex<T>` returns a _smart pointer_ of type _MutexGuard_. _MutexGuard_ implements _Deref_ to allow us a to acquire a reference to the inner data. _MutexGuard_'s implementation of _Drop_ automatically releases the lock when the _MutexGuard_ goes out of scope. This prevents us from forgetting to release the _lock_. `Mutex<T>` also provides interior mutability like `RefCell<T>`, so you can reference it in an immutable reference and still mutate the value it holds.

Mutexes have two usage rules:

- You must attempt to acquire the lock before using the data.
- When you're done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

##### Acquiring the lock

The type system won't let us use the value held by the mutex before we acquire the lock.

To acquire the lock of the `Mutex<T>` you call `m.lock()` which will block the current thread until the lock is available, then return a `Result<MutexGuard<T>, PoisonError<E>>`.

If a thread holding the lock panics, the call to `m.lock()` will fail, preventing from any other thread from acquiring the lock. 

##### Unlocking the data.

As stated above, since the _Mutex_ will return a _MutexGuard_ when we call `m.lock`, and `MutexGuard#drop` will automatically unlock the mutex when it goes out of scope. We can usually count on the _Mutex_ to be unlocked automatically.

#### Sharing Mutexes between threads

See examples `share_data_between_two_threads_with_mutexes` and `share_data_between_many_threads_with_mutexes` in `chapter_16/src/main.rs`.

We can't use `move` to move a mutex into closures going to two differnent threads, this will result in a compiler error: 

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

The answer is to enable ultiple ownership with a _reference counting pointer_. We might try to use `Rc<Mutex<T>>`, but we would get another error: 

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

_deadlocks_ occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other to release the other lock forever. A program with 2 or more threads, and two or more `Mutex<T>`s could fall victim to _deadlocks_.

**Challenge** Create a program with _deadlocks_, then research _deadlock_ mitigation strategies for mutexes to fix the program. Refer to the docs for `Mutex<T>` and `MutexGuard` for useful information.

### Extensible Concurrency with the Sync and Send Traits

Most concurrency features come from the standard library, not the core Rust language. This is evident by all the imports made in `chapter_16/src/main.rs`. Two concurrency concepts are enbedded in the language, `std::marker` traits `Sync` and `Send`

`std::marker` is a module of primitive traits and types representing basic properties of types.

#### Allowing Transference of Ownership Between Threads with Send

Types that implement `Send` can be moved between threads. `Rc<T>` is not `Send`, because two or more threads could try to update the reference count at the same time. Any type composed entirely of `Send` types is automaticaly marked `Send`. 

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

For sharing data by message passing: use `std::sync::mpsc#channel` with `std::sync::mpsc::{Sender, Receiver}`.

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
        String::from("hello world")
    }
}

struct Foo {
    biz: i32,
}

impl Summary for Foo {}
```

You can also override default implementations by redefining them for your type.

If you wanted to match the pattern of calling `super` then performing some other work in your override function, you could use a conditional implementation where you impl the trait for types that implement some other trait. That way you could call methods from that other trait in your override method

2. The other reason to use inheritance is to allow a child type to be used in the same places as the parent type. We usually say that code that can work with data of multiple types is _polymorphic_. Rust uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. This is referred to as _bounded parametric polymorphism_.

Rust uses _trait objects_ instead of inheritance to avoid two common inheritance problems:

1. Sharing too much code.
2. Only being able to inherit from one parent. (a type enclosed in a trait object could implement the trait the trait object uses, but also implement any other number of traits)

## Using Trait Objects that Allow for Values of Different Types.

In Chapter 8 we defined an enum `SpreadsheetCell` to hold *integer*, *float* and *text* variants in the same `Vec`. We did not know which enum variants would be in which cells in the `Vec` at compile time, but because knew we had a fixed set of `SpreadSheetCell` types, we were able to use an `enum` to get the code to compile.

If we wanted our library user to be able to extend the set of types they can use with our API, an enum will not work, because it cannot be extended without modifying the API.

We would need to use _trait objects_. A _trait object_ points to an instance of a type that implements the trait we specify. We create a _trait object_ by specifying a pointer `&` or `Box<T>` and specifying the relevant trait. Rust can ensure at compile time that any value used in a trait object will implmement the trait object's trait.

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
    pub component: Box<dyn Draw>,
}
```

Notice that we don't need to define generic type parameters. Generic type parameters guarantee that we only substitute one concrete type at a time, for each paramter. So everywhere you see `T` we're talking about the same concrete type. With _trait objects_ we're allowed to have multiple concrete types fill in for the object, so we don't need to specify a generic type paramter for the implementation block or the method. 

When you know a collection is going to be homogoneous, you should prefer to use generics and trait bounds to allow for monomorphization, which is the compiler's behavior of filling in concrete types at compile time. With _trait objects_, the compiler doesn't know what types will be filled in, so monomorphization is impossible.

The idea of being concerned with the messages a value responds to rather than the value's concrete type is similar to _duck typing_. Our `Screen#run` method in `chapter_17/src/lib.rs` leverages _duck typing_ in that it doesn't care what each component's type is, or how it implements `Draw`. The advantage Rust has over most languages that implement _duck typing_ is that it still will not compile if it is possible for a message (method) to be sent (called) when the value is of a type that doesn't implement it.

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

While _trait objects_ allow us some flexibibility over using generics, there is a runtime cost. The running program must spend time finding the right method to call based on the type of the value enclosed in the _trait object_.

### Object Safety is Required for Trait Objects

Only _object-safe_ traits can be made into trait objects. A trait is _object-safe_ if all the methods defined in the trait have the following properties:

- the return type isn't Self
- There are no generic type parameters

Once you have created a _trait object_, the running program no longer knows the concrete type that's implementing that trait.

trait objects do not remember the concrete type of the values that they hold. So you wouldn't be able to call a trait method that returns `Self` on a trait object, because it would not know what type to return. Similarily, a trait method with a generic type parameter could not be called by a trait object, because it would not know what concrete type to fill in.

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

Here we see that a "vtable" entry cannot be made for `clone` for a typebecause the return type of `Clone::clone` cannot be resolved dynamically, since the trait object does not know anything about the concrete types it holds.

**Question**: We now know that a vtable has to be made for trait objects to allow for dynamic dispatch. The compiler cannot perform static dispatch on arbitrary values that might go into the trait object. As such, a vtable needs to be created for each value that is used in the trait object, so the running program knows what methods to call on that value. Why, instead of a vtable, can't the trait object just hold store the concrete type of the value, so we can find the methods defined on that value at runtime?

Answer: Basically when you have a block of memory in a running program, there is no information about what type it is. The compiler guaranteed that it will only be used in a valid way, but there is no way to determine what type it is. To store type information about each block of memory would create a lot of extra overhead.

From https://stackoverflow.com/questions/67767207/why-are-trait-methods-with-generic-type-parameters-object-unsafe, there is the comment:

```
Having a pointer to some memory is not the same as knowing specific information about the type of data being pointed at. vtables are a means of representing type-related data for use at runtime. In the case of returning Self, your call to echo() allocates space on the stack for the return value of the function, before the funciton is called, and that size is determined at compile time, not at run time. Variable size stack allocations are pretty rare, and having to look at a vtable to decide how much stack size to allocate would also be a lot of additional complexity, and slow things down. – 
loganfsmyth
 May 31 2021 at 20:52
 ```

### Implementing an Object-Oriented Design Pattern

In addition to being able to hold values of arbitrary type, trait objects have the advantage over enums in that each type your trait object can hold can have its own implementation. That is, each type that implements the trait is responsible for its own implementation. With enums, the variants cannot have their own implementation, they can only hold types that have their own implementation. 

Because trait objects allow us to hold any type that implements a certain trait, they are perfect for implementing a _state pattern_. We can use a trait object to hold _state objects_, which are values that can represent a certain state, and implement behavior that is related to that state.

An advantage of the _state pattern_ is that it allows for a separation of concerns. Any data structure holding a state object does not need to know the rules of how to manipulate (mutate) the state. That logic can be encoded in the _state objects_ themselves, so if the rules around how the state can be mutated change, the data structures that hold the _state object_ do not need to change, they can just continuing passing messages to the state objects.

#### The `#take` Method

The `#take` method on `Option<T>` returns the enclosed `Some<val>` value and leaves a `None` value in its place. 

This is useful when there is a mutable reference to something holding an `Option<T>` and we want to transfer ownership of the enclosed `Some<T>` value. By replacing the `Some<val>` with `None`, we are able to acquire ownership of `val` without violating the borrowing rules that guarantee the reference will remain valid.

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

## Chapter 18 (Patterns and Matching)

### While let

Run a loop until the match fails.

```
let mut stack = vec![];

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### let statements

All `let` statements use patterns, just as `match` and `if let`. Unlike `if let`, `let` must be exhaustive:

```
    let v = vec!['a', 'b', 'c'];

    let Some((i, val)) = v.iter().enumerate().next();

    println!("{} is my index, {} is my value", i, val);
```

This is a case where we would have to use `if let` to ignore the `else` case where the pattern can't match a `None`:

```
error[E0005]: refutable pattern in local binding: `None` not covered
   --> src/main.rs:21:9
    |
21  |     let Some((i, val)) = v.iter().enumerate().next();
    |         ^^^^^^^^^^^^^^ pattern `None` not covered
    |
   ::: /Users/carsonrajcan/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:514:5
    |
514 |     None,
    |     ---- not covered
    |
    = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
    = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
    = note: the matched value is of type `Option<(usize, &char)>`
help: you might want to use `if let` to ignore the variant that isn't matched
    |
21  |     if let Some((i, val)) = v.iter().enumerate().next() { /* */ }
    |
```

### Multiple Assignment

We can assign to multiple variables at once using a pattern:

`let (x,y,z) = (1,2,3);`

If the pattern on the left and the value on the right have different types we'll get a mismatched types error:

`let (x,y) = (1,2,3);`

```
error[E0308]: mismatched types
  --> src/main.rs:34:9
   |
34 |     let (x, y) = (1, 2, 3);
   |         ^^^^^^   --------- this expression has type `({integer}, {integer}, {integer})`
   |         |
   |         expected a tuple with 3 elements, found one with 2 elements
   |
   = note: expected tuple `({integer}, {integer}, {integer})`
              found tuple `(_, _)`
```

Ignore a value: `let (x,_,z) = (1,2,3);`

Match in Function params:

```
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("current location: ({}, {})", x, y);
}
```

This also works in closures:

```
let c = |&(x, y)| {
    println!("current location: ({}, {})", x, y);
};

c(&(3, 5));
```

### Refutability

Patterns that will match for all possible values passed are _irrefutable_. 
Patterns that can fail to match for some possible value are _refutable_.

Function paramters, `let` statements, and `for` loops can only accept _irrefutable_ patterns because the program would not know what to do if the patterns did not match.

`if let` and `while let` expressions can only accept _refutable_ patterns, because they are designed to handle the failure condition from a match.

Similar to above:

```
let foo = Some(1);
let Some(x) = foo;
```

The compiler cannot can not know that this will always match, since foo could be passed in by the user

```
error[E0005]: refutable pattern in local binding: `None` not covered
```

Similarily if we try to use an _irrefutable_ pattern where a _refutable_ one is expected: 

```
let foo = 1;

if let x = foo {
    println!("x is {}", x);
}
```

This has been downgraded to a warning, It's like saying `if true` and shaddowing the value `foo`. Also:

```
let foo = 1;

match foo {
    x => println!("foo"),
}
```

### Pattern Syntax

#### Matching literals: 

```
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("larger"),
}
```

#### Matching named variables:

```
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
}
```

This will match any `Some(val)` where `val` is not `50`, and bind `val` to the variable `y`, which is shadowed in the scope of the match. After the match expression ends, `y` is again `10`.

#### Multiple Patterns in One Match Arm

```
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

#### Matching Ranges

We can match an inclusive range of chars or numerics:

```
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
```

```
let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("later ASCII letter"),
    _ => println!("something else"),
}
```

Exclusive ranges are **EXPERIMENTAL** in match arms! Probably because it doesn't make sense to have a match arm list a value you don't want to match!. `1..5 => println!("foo");` would not match 5, which is confusing.

#### Destructuring

##### Structs

```
let p = Point { x: 0, y: 7 }
let Point { x: a, y: b } = p;

assert_eq!(0, a);
assert_eq!(7, b);
```

or if you want your new variable to match the field names of the struct:

```
fn print_coordinates(p: Point) {
    let Point { x, y } = p;
    println!("I'm at {}, {}!", x, y);
}
```

```
fn which_axis(p: &Point) {
    match p {
        Point { x: 0, y: 0 } => println!("On both axis at ({},{})", p.x, p.y),
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y: _ } => println!("On neither axis!"),
    }
}
```
Notice in the last one we explicitly said `y` can be anything without matching its value to any variable.


##### Enums

```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}
```

**Challenge** Figure out why the `chapter_18/src/main.rs#challenge_str` function doesn't compile.

Answer: You can't bind a str to a local var because it doesn't have a size known at compile time.
It is unsized, as is [T].

**Challenge** Figure out why the `chapter_18/src/main.rs#challenge_string` function doesn't compile.

Answer: You're essentially trying to move the value pointed to by x into y. But x doesn't own the value so you can't transfer ownership from it.

**Challenge** figure out why `chapter_18/src/main.rs#destructuring_references` doesn't work when we try to deref in the match. 
Answer: The difference here is that in the closure, we are specifying that
the argument should be a reference to a `Point`, and then we are copying
 x & y. In the match expression, we are trying to transfer ownership of the
value pointed to by the &Point from the array to the variable 'p'.

Notice when we try to use the same pattern from some_of_squares in
"sum_of_strings" we get the same "cannot move out shared reference" error

**Challenge** Try to use destructuring to avoid having to use `#as_ref` as we did in the examples in chapter 17.

Answer: it looks like it is possible:

```
// This is match ergonomics again
if let StringHolder { val: Some(s: &String) } = holder_ref {
    println!("capacity of string: {}", s.capacity());
}
```
See `chapter_18/src/main.rs#borrow_nested_data_that_is_borrowed`.

**Follow up**: So when are `ref` and `ref mut` used for destructuring? Use them to destructure The `&StringHolder` and avoid using `as_ref` in `chapter_18/src/main.rs#borrow_nested_data_that_is_owned`

It turns out we need to use `ref` when destructuring to get a reference into nested date of an _owned_ value. When the value is borrowed, we can get a reference into its nested data by providing a non-reference pattern, because of _match ergonomics_.

#### Ignoring Entire Values With _

```
fn foo(_: i32, y: i32) {
    println!("This code only usees the y parameter: {}", y);
}
```

The compiler will usually suggest to do this

#### Ignoring Parts of a Value with a Nested _

```
let p = Point { x: 1, y: 3 };
let Point { x: a, y: _ } = p;
println!("a is {}", a);
```

#### Ignoring usused variables with a leading _

Compiler warning: 

```
fn main() {
    let x = 5;
}
```

No compiler warning: 

```
fn main() {
    let _x = 5;
}
```

Prints "_x = 5":

```
fn main() {
    let _x = 5;
    println!("_x = {}", _x);
}
```

Value borrowed after move:

```
fn main() {
    let _x = 5;
    takes_ownership(_x);
    println!("_x = {}", 5);
}
```

So using a leading `_` still causes the variable to bind to the value!

#### Ignoring Remaining Parts of a Value with ..

With structs:

```
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point {x, .. } => println!("x is {}", x):
}
```

with tuples:

```
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, .., last) => {
        println!("Some numbers: {}, {}", first, last);
    },
}
```


with vectors: 
```
let numbers = vec![2, 4, 8, 16, 32];

match numbers[..] {
    [first, second, .., last] => {
        println!("Some numbers: {}", second)
    }
    _ => println!("too many numbers!"),
}
```

##### Using .. in patterns must be unambiguous:

```
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}

```

We see the resulting error:

```
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error: `..` can only be used once per tuple pattern
 --> src/main.rs:5:22
  |
5 |         (.., second, ..) => {
  |          --          ^^ can only be used once per tuple pattern
  |          |
  |          previously used here
```

### Creating References in Patterns with ref and ref mut

#### Creating an Immutable Reference While Destructuring.

When we want to avoid a _partial move_ (taking ownership of a nested piece of data), we can create a reference to the nested data.

When trying to create a reference into a nested data structure that is owned, we might have the inclination to destructure using `&x`, however, when used in destructuring patterns, `&` does not _create_ a reference, but actually matches an existing reference (this only really works with values that are copy):

```
let robot_name = Some(String::from("Bors"));

match robot_name {
    some(&name) => println!("Found a name: {}", name);
}

println!("robot_name is: {:?}", robot_name);
```

Using `&` here will yield an error: 

```
error[E0308]: mismatched types
   --> src/main.rs:234:14
    |
233 |     match robot_name {
    |           ---------- this expression has type `Option<String>`
234 |         Some(&name) => println!("Found a name: {}", name),
    |              ^^^^^
    |              |
    |              expected struct `String`, found reference
    |              help: you can probably remove the explicit borrow: `name`
    |
```

The solution is to use the `ref` keyword to create a reference while destructuring: 

```
let robot_name = Some(String::from("Bors"));

match robot_name {
    some(ref name) => println!("Found a name: {}", name);
}

println!("robot_name is: {:?}", robot_name);
```

#### Creating a mutable reference while restructuring.

Similarily, we cannot use `&mut` while destructuring to create a mutable reference into nested mutable owned data. `&mut` can only be used to match existing mutable references.

```
let mut robot_name = Some("borg".to_string());

match robot_name {
    Some(&mut name) => *name = "blork".to_string(),
    None => ()
}

println!("robot_name: {:?}", robot_name);
```

Just like we saw above, this will not compile because `robot_name` is an `Option<String>`, not an `Option<&mut String>`:

```
error[E0308]: mismatched types
   --> src/main.rs:234:14
    |
233 |     match robot_name {
    |           ---------- this expression has type `Option<String>`
234 |         Some(&mut name) => *name = Some("blork".to_string()),
    |              ^^^^^^^^^
    |              |
    |              expected struct `String`, found `&mut _`
    |              help: you can probably remove the explicit borrow: `name`
    |
    = note:         expected struct `String`
            found mutable reference `&mut _`
```

We can fix this by using `ref mut`:

```
let mut robot_name = Some("borg".to_string());

match robot_name {
    Some(ref mut name) => *name = "blork".to_string(),
    None => (),
}

println!("robot_name: {:?}", robot_name);
```

**note** the only way to use `&` or `&mut` in a pattern to actually match a value is if the nested value you are trying to match implements `Copy`. Otherwise you are attempting to take ownership of a shared reference

**Will Compile**:

```
let v = vec!["foo", "bar"];

match v.iter().next() {
    Some(&x) => println!("got x"),
    None => println!("got x"),
}

println!("v: {:?}", v);
```

`&str` implements `Copy`

**Won't Compile**:

```
let v = vec!["foo".to_string(), "bar".to_string()];

match v.iter().next() {
    Some(&x) => println!("got x"),
    None => println!("got x"),
}

println!("v: {:?}", v);
```

`String` does not implement `Copy`

### Extra Conditionals in Match Guards

Create a match guard by appending an `if` expression to a pattern. Match guards allow us to compare things that can't be expressed using a pattern alone.

```
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("x: {}", x),
    None => (),
}
```

Match guards can also be used to compare values bound to variables from the outer scope:

```
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50),
    Some(n) if n == y => println!("Matched, n = {:?}", x),
    _ => println!("Default case, x = {:?}", x)
}
```

#### | with match guards

When an `|` is used in a pattern, the `|` expression takes precedence over a match guard:

```
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```

This will print "no" because the `if y` applies when `x` is `4`, `5`, or `6`, not just when `x` is `6`.


We cannot transfer ownership (move) it a match guard:

```
match foo {
    Some(x) if x + "bar" == "foobar".to_string() => println!("got part of foobar"),
    Some(x) => println!("string is not part of foobar"),
    None => println!("got nothing"),
}
```

This will result in an error:

```
error[E0507]: cannot move out of `x` in pattern guard
   --> src/main.rs:339:20
    |
339 |         Some(x) if x + "bar" == "foobar".to_string() => println!("got part of foobar"),
    |                    ^ move occurs because `x` has type `String`, which does not implement the `Copy` trait
    |
    = note: variables bound in patterns cannot be moved from until after the end of the pattern guard

```

Similarily if we try to borrow as mutable:

```
// We also cannot borrow as mutable in a pattern guard
match foo {
    Some(x) if x.as_bytes_mut() == [b'f', b'o', b'o', b'b', b'a'] => println!("got fooba"),
    Some(x) => println!("string is not fooba"),
    None => println!("got nothing"),
}
```

```
error[E0596]: cannot borrow `*x` as mutable, as it is immutable for the pattern guard
   --> src/main.rs:349:20
    |
349 |         Some(x) if x.as_bytes_mut() == [b'f', b'o', b'o', b'b', b'a'] => println!("got fooba"),
    |                    ^ cannot borrow as mutable
    |
    = note: variables bound in patterns are immutable until the end of the pattern guard
```

### @ Bindings

The `@` (_at_) operator lets us bind a value to a variable while why are also testing it with a pattern:

```
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
```

Here we bind the `id` field to the variable `my_id` while we check if the `id` value is between 3 & 7. 

It is perhaps more concise to just use match guards:

```
match msg {
    User { id } if (3..=7).contains(&id) => println!("Found an id in range: {}", id),
```

### Match Ergonomics:

A new RFC made things a lot cleaner. Matching on a reference used to require explicit destructuring using `&` and `*`:

```
let x: &Option<_> = &Some(0);

match x {
    &Some(ref y) => { ... },
    &None => { ... },
}

// or using `*`:

match *x {
    Some(ref x) => { ... },
    None => { ... },
}
```

With a new RFC, the above form still works, but now we also allow a simpler form:

```
let x: &Option<_> = &Some(0);

match x {
    Some(y) => { ... }, // `y` is a reference to `0`
    None => { ... },
}
```

The expression being matched is a reference (type &Option) but the patterns are not reference patterns (`ref`, `ref mut`). This is where match ergonomics kick in: the rule says that when a reference is matched with a non-reference pattern, the bindings within that pattern bind by reference rather than by value (i.e. as if they were prefixed with ref).

This also means that we only need to use `ref` and `ref mut` when trying to borrow owned data via destructuring. When trying to borrow from borrowed data via destructuring, the compiler will automatically add `ref` and `ref mut` behind the schenes where appropriate.

The following functions in `chapter_18/src/main.rs` demonstrate when to use `ref` and `ref mut`, when they can be avoided, and when we need to use `.as_ref`.

```
borrow_nested_data_that_is_owned();
borrow_nested_data_that_is_borrowed();
mutably_borrow_nested_data_that_is_owned();
mutably_borrow_nested_data_that_is_mutably_borrowed();
``` 

## Chapter 19 (Advanced Features)

### Unsafe Rust 

#### Unsafe Superpowers

- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
  
`unsafe` does not turn off the borrow checker. 

It's best to enclose `unsafe` code within an abstraction and provide a `safe` API.

#### Dereferencing a Raw Pointer

Rust has two _raw pointer_ types, `*const T` and `*mut T`. 

_Raw pointers_ have some capabilities that regular _references_ and _smart pointers_ do not:

- Ignore the borrowing rules by having both immutable and mutable pointers, or multiple mutable pointers to the same location.
- Aren't guaranteed to point to valid memory (during a pointer's lifetime, some other pointer could mutate or drop the value it points to)
- Are allowed to be null
- Don't implement any automatic cleanup (we need to call free presumably)

Sounds a lot like C pointers...

Creating raw pointers;

```
let mut num = 5;
let r1 = &num as *const i32;
let r2 = num as *mut i32;
```

here `as` is used to cast normal references into _raw pointers_.

We can create a raw pointer to any address:

```
let address = 0x012345Usize;
let r = address as *const i32;
```

**Challenge** use `Rc<T>` to create multiple owners of a piece of data, then try to mutate that data with a mutable reference

Answer: This is not possible. The `Rc<T>` consumes (takes ownership of) the enclosed value on creation. See `chapter_19/src/main.rs#rc_challenge`.

#### Creating a Safe Abstraction over Unsafe Code

Suppose we want to implement a the function `#split_at_mut` which will create two mutable slice references to two segments of a given slice, given a split point:

```
let mut v = vec![1, 2, 3, 4, 5, 6];
let r = &mut v[..];
let (a, b) = split_at_mut(r, 3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);
```

Trying to implement this in safe rust would look like this:

```
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    assert!(mid <= slice.len());

    (&mut slice[..mid], &mut slice[mid..])
}
```

**Note** in the expression `(&mut slice[..mid], &mut slice[mid..])`, the `..` notation is inclusive on the lower bound and exclusive on the upper bound. If we call `slice[n..]` where n is equal to the length of the slice, we will get back nothing. If we call `slice[..n]` where n is equal to the length of the slice, we will get back the entire slice.

This won't compile because we're trying to create two mutable references to the same value:

```
error[E0499]: cannot borrow `*slice` as mutable more than once at a time
  --> src/main.rs:59:30
   |
56 | fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
   |                        - let's call the lifetime of this reference `'1`
...
59 |     (&mut slice[..mid], &mut slice[mid..])
   |     -------------------------^^^^^--------
   |     |     |                  |
   |     |     |                  second mutable borrow occurs here
   |     |     first mutable borrow occurs here
   |     returning this value requires that `*slice` is borrowed for `'1`
```

**Challenge** Figure out why we can't compare `&[T]` with `[T]`

Answer: see `chapter_19/src/main.rs#compare_slices`.

#### Using `extern` Functions to Call External Code

`extern`: A keyword for facilitating the creation of a _FFI_
_Foreign Function Interface (FFI)_: A way for a programming language to define functions and enable a different (foreign) programming lanugage to call those functions.

Functions declared with `extern` blocks are always considered `unsafe` since the Rust compiler has no way to check code from other languages.

```
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}, abs(-3));
    }
}
```

Here, `extern "C"` specifies that we want to use the C _application binary interface (ABI)_, which defines how to call the function at the assembly level. The "C" _ABI_ is the most common and follows the C programming language's ABI.

                                  /\ 
                                 /__\
                                  ||
                                  || 
                                  || 
 **Challenge**: Actually use this || 

 Answer:: See `chapter_19/src/main.rs#calling_c`.

#### Calling Rust Functions From Other Languages

**Challenge**: Actually use this:

```
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a rust fn from C");
}
```

Answer: See `chapter_19/src/lib.rs` and  [this](https://docs.rust-embedded.org/book/interoperability/rust-with-c.html)

1. Create your function in `chapter_19/src/lib.rs`.
2. Run cargo build with the `[lib]` options specified by `chapter_19/Cargo.toml`. It must include `crate-type = ["cdylib"]` or `crate-type = ["staticlib"]`.
3. Run cbindgen with options listed in `chapter_19/src/lib.rs`.
4. Create a `rust_fn.c` with `include "rust_fn.h"` and an `int main()` function that calls `call_from_c()`.
5. Run `gcc rust_fn.c target/debug/librust_fn.dylib` or `gcc rust_fn.c target/debug/librust_fn.a`, depending on if you created a dynamic lib or a static lib with the `[lib]` options in `Cargo.toml`.

#### Accessing or Modifying a Mutable Stack Variable

```
static HELLO_WORLD &str = "Hello, World!";

fn main() {
    println!("message is {}", HELLO_WORLD);
}
```

Unlike constants which duplicate their memory every time they are used, immutable static variables have a fixed address.

**Challenge** Prove this with an example ^^^

Answer: What this means is that every occurance of a constant will be inlined in the binary, while static variables will only be stored in the binary once, and can be modified at runtime.

Static variables can also be mutable:

```
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
```
Any code that reads or writes to a mutable static must be in an unsafe block, because the compiler cannot guarantee there will not be a data race without a `Arc<Mutex<T>>` (remember the `Mutex<T>` locks to allow prevent data races and the `Arc<T>` allows for shared ownership across threads).

#### Implementing an Unsafe Trait.

_`unsafe` trait_: One that the compiler can't verify due to some invariant

```
unsafe trait Foo {

}

unsafe impl Foo for i32 {

}
```

`Sync` and `Send` are marker traits that the compiler implements automatically if our types are composed entirely of `Sync` or `Send` types. If we implement a type that contains a type that is not `Send` or `Sync` and we want to mark it (implement) `Send` or `Sync` we must use `unsafe` because the compiler cannot verify that the type can be safely sent across threads or accessed from multiple threads.

### Advanced Lifetimes

 - _Lifetime subtyping_: Eunsures that one lifetime outlives another lifetime. (Remember we usually specify one lifetime will be at least as long as another by labeling two varables with the same lifetime parameter).

 - _Lifetime bounds_: Specifies a lifetime for a reference to a generic type.

 - _Inference of trait object lifetimes_: Allows the compiler to infer trait object lifetimes and when they need to be specified.

#### Ensuring One Lifetime Outlives Another with Lifetime Subtyping

Remember that _input lifetimes_ are lifetimes on function or method parameters. _Output lifetimes_ are lifetimes on return values.

We have used lifetime parameters before when we had a return value that referenced a value referenced by an argument. 

We were able to specify that the _output lifetime_ would be at least as long as the _input lifetime_ it was derived from.

Now suppose we have a function that creates an owned value that references an input reference, then we call a method on this owned value to return a reference to the input reference.

**OUTDATED** The compiler can tell as long as the input reference and the owned value's reference have different lifetimes.

Since the input reference was encapsulated by a value that is about to go out of scope, the compiler cannot tell that the _output lifetime_ will be at least as long as the function parameter. Because it cannot tell that the _output lifetime_ was derived from the function paramter due to the layer of indirection (the owned value that is going out of scope).

```
struct Context<'a>(&'a str);

struct Parser<'a> {
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    fn parse(&self) -> Result<(), &str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(c: Context) -> Result<(), &str> {
    Parser { context: &c }.parse()
}
```

In this example we create an owned value that holds a reference to the input parameter, and then later we use `#parse()` to a return a reference that was held by the input paramter

Due to the indirection created by wrapping the input parameter in a local owned value, the compiler doesn't realize that we are returning a reference that was passed in by the owned input paramter:

```
error[E0515]: cannot return value referencing function parameter `c`
   --> src/main.rs:215:5
    |
215 |     Parser { context: &c }.parse()
    |     ^^^^^^^^^^^^^^^^^^--^^^^^^^^^^
    |     |                 |
    |     |                 `c` is borrowed here
    |     returns a value referencing data owned by the current function

error[E0515]: cannot return value referencing temporary value
   --> src/main.rs:215:5
    |
215 |     Parser { context: &c }.parse()
    |     ----------------------^^^^^^^^
    |     |
    |     returns a value referencing data owned by the current function
    |     temporary value created here
```


Notice if we were to avoid encapsulating `c` in a `Parser`, the compiler would be fine with us returning a reference that was owned by the input paramter:

```
fn parse_context(c: Context) -> Result<(), &str> {
    Err(&c.0[1..])
}
```

We need to tell the compiler that the reference held by the input paramter, is not going to be dropped when the `Parser` is dropped. So we can try to specify that have different lifetimes by annotating `Parser`:

```
struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}
```

And this appears to be enough for the compiler to understand that `c.context` will not be dropped when the local `Parser` instance gets dropped.

Apparently _lifetime subtyping_ is not needed (atleast here), and this is an instance of the book being out of date. The way the book suggests fixing this is by specifing that `'s` will outlive `'c`:

```
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}
```

**Challenge** determine why _lifetime subtyping_ is no longer needed here and if it is still a feature to be used elsewhere. Refer to `error[E0491]` (`rustc --explain E0491`)

Possible answer: It looks like maybe it is due to NLL. The compiler can now tell that `c.context` isn't necessarily going to be dropped with the local `Parser` instance.

#### Lifetime Bounds on References to Generic Types

_lifetime bounds_ are liftetime parameters that the compiler can use to verify that the references in generic types won't outlive the data they're referencing.

Recall that when we defined a type that holds a reference to a type that also holds references, we needed to use lifetime parameters to specify that the references held by the value we are pointing to need to live at least as long as the reference in our type:

```
struct Foo<'a>('a str);

struct FooHolder<'a> {
    foo: &'a Foo<'a>
}
```

Here we specified that `FooHolder.foo` (the reference) will live at least as long as the string slice held by the `Foo` it points to.

But what if `FooHolder` held a reference to a generic type `T`?

```
struct FooHolder<'a> {
    foo: &'a T 
}
```

The compiler appears not to care about this either. Probably due to NLL, it can now tell that references in the generic type will live at least as long as the reference to the generic type. 

The way the books recommends fixing this is as follows:

```
struct FoohHolder<'a, T: 'a>(&'a T)
```

This says that `T` can be any type, but if it contains references, they must live at least as long as 'a

**Challenge** determine why _lifetime bounds_ is no longer needed here and explain the context that they still need to be used as explained by `error[E0309]` (`rustc --explain E0309`)

Answer: It looks like this is really only applicable when creating a _trait object_ where that _trait_also has lifetime bounds. The compiler error tells you how to fix it. 

```
struct Foo<'a, T> {
    foo: <T as SomeTrait<'a>>::Output,
}

trait SomeTrait<'a> {
    type Output;
}

impl<'a, T> SomeTrait<'a> for T
where
    T: 'a,
{
    type Output = u32;
}
```

The compiler error tells us just to specify `T: 'a`, (`T` lives at least as long as `'a`) in our definition of `Foo`.

#### Inference of Trait Object Lifetimes

We used `Trait Objects` before to create references to any type that implements a specific trait. This allowed us to use dynamic dispatch on the methods on that type.

We did not cover the case where the type that implements the trait has a lifetime (reference) of its own.

```
trait Red {}

struct Ball <'a> {
    diameter: &'a i32
}

impl <'a> Red for Ball<'a> {}

fn use_trait_object_with_lifetimes() {
    let num = 5;

    let obj: Box<dyn Red> = Box::new(Ball { diameter: &num});
}
```

This compiles without any errors due to the compiler's rules for working with lifetimes and trait objects:

- The default lifetime of a trait object is `'static`
- With `&'a Trait` or `&'a mut Trait`, the default lifetime of the trait object is `'a`.
- With a single `T: 'a` clause, the default lifetime of the trait object is `'a`.
- With multiple clauses like `T: 'a`, there is no default lifetime; we must be explicit.

### Advanced Traits

#### Specifying Placeholder Types in Trait Definitions with Associated Types

_Associated types_ connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.

```
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>
}
```

The trait implementor will specify the concrete type to be used as the associated type for a particular `impl` 

#### Default Generic Type Parameters and Operator Overloading

When implementing a trait, we can specify default concrete types so that the implementor of a trait does not have to specify a concrete type if the default type works:

```
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

here we have specified that the type of the `rhs` argument to our `add` method will default to `Self`, the type the trait is implemented for. Now when someone comes along to implement the trait for their type, they can skip specifying a generic type paramter on the `impl` block:

```
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.y,
            y: self.y + rhs.y
        }
    }
}
```

Notice that because the trait definition for `Add` specifies that the `rhs` argument defaults to `Self`, we did not have to specify another type for the generic type parameter that is used as `rhs`.

If we wanted to impl add for our type to be added to some other type, we could override the default generic type paramter defined on `Add` by supplying our own:

```
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Millimeters {
        //snip
    }
}
```

Here we want to define what happens when we add a value stored in `Meters` to a value stored in `Millimeters`. Because `Add` is defined to default to adding two values of the same type, we overrode the generic type paramter for the `rhs` argument with the type `Meters`. Notice that in this we fill in the concrete type on the trait itself, not on the `impl` block or the type we are implementing the trait for.

```
impl<T> Bark for Container<T>
```

Usually in the past we specified that we were creating a generic impl for a generic type.

There are two main use cases for default type parameters:

- Extending a type without breaking existing code.
- To allow customization in special cases most users won't need (like in the case of `Add`)

An example of the first use case would be if you had the `Add` trait, and it only supported added two values of the same type. If you wanted to _then_ support added two different types, you can avoid breaking existing code by using the default type parameter. This way you don't have to then update every other `impl` block for `Add` to specify the generic type paramter for `rhs`.

#### Operator Overloading

We've seen here that we can overload operations and traits listed in `std::ops` by implementing the traits associated with the operator. We can overload `+` by implementing `Add`.

#### Calling Multiple Methods From One Type With The Same Name

Nothing prevents you from implementing two traits for a type that use the same method name, or implementing a method directly on a trait that has the same name as some trait method.

```
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking);
    }
}

impl Wizard for Human {
    fn fly(&self) {
        ptinln!("Up");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

The compiler will default to calling the method defined directly on the type. This means `person.fly()` will call the method from the `impl Human` block. To call the methods from the trait implementations, we need to call `Pilot::fly(&person);` and `Wizard::fly(&person);`. Using the more explicit syntax allows us to call methods from two different trait implementations for the same type.


#### One Type Two Traits with Overlapping Associated Function Names

If we are using methods on instances of those types, the compiler can figure out what method to call because they take a `self` parameter. If we are trying to use associated functions from those two trait implementations, and both types are in the same scope, we need to use _fully qualified syntax_.

```
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

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

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name()); //Spot
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); //Puppy
}
```

Since we are not passing an instance of `Dog` to either method, we need the prefix `<Dog as Animal>` to access the `baby_name()` frunction from the implementation of `Animal for Dog`.

**Challenge**: Figure out how to resolve ambiguity when two types implement the same trait 

Answer: This is actually unambiguous:

```
Dog::baby_name();
Cat::baby_name();
```

#### Using Supertraits to Require One Trait's Functionality Within Another Trait

Suppose we want to define a trait that can only be implemented by types that define another trait.

Remember previously we had defined trait implementations for any type that implements some other type:

```
// "Implement ToString for T when T is Display
impl<T: Display> ToString for T {
}
```

This is similar but more strict, in this case we are defining the trait in such a way that _only_ types that implement some other _supertrait_ can implement the _dependent trait_:

```
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
```

See the example usage in `chapter_19/src/main.rs#use_supertraits`.

*Note*: When we have a _dependent trait_ that requires a _super trait_, but our type does not implement the _super trait_ for all generic _enclosed_ types, we can still implement the _super trait_ for all enclosed types that implement the _super trait_, and then implement the _dependent type_ only when our _enclosed type_ implements the supertrait:

```
impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: fmt::Display> OutlinePrint for Point<T> {}
```

Meaning, when the _dependent trait_ says "you have to be able to do X", and our type can't always do "X" because of its _enclosed types_, we can still use the _dependent trait_ on our type in the cases where our _enclosed types_ _can_ do "X".

This is clever, lol.

#### Using the NewType Pattern to Implement External Traits on External Types

_orphan rule_: we can implement a trait on a type only if either the trait or the type are local to our crate.

_newtype pattern_: creating a new wrapper tuple struct type to enclose an external type just to implement the trait for the wrapper type.

Both `Display` and `Vec<T>` are external types in the context of our crate.

```
use std::fmt;

struct Wrapper(Vec<string>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

Notice this only works in this case because the `Vec<T>` is specified to be a `Vec<String>`, and `String` implements `Display`.

Unfortunately, the _newtype_ would not have all the methods of the value it's holding. We would have to reimplement them all manually on the wrapper type by deferring to `self.0`. We could give the wrapper type all the methods of the inner type by implementing `Deref` on the wrapper type.

### Advanced Types

#### Using the NewType Pattern for Type Safety and Abstraction

Newtypes are useful for carrying the units of a value:

```
struct Millimeters(i32);
struct Meters(i32);
```
Newtypes are also useful for abstracting away some implementation details of a type.

```
struct People(Hashmap<i32, String>);
```

If we keep the inner values private, we can achieve _encapsulation_: to keep the implementation details of our interface hidden from the users of our interface.

#### Creating Type Synonyms with Type Aliases

```
type Kilometers = i32;
```

This declaration defines `Kilometers` to be the same type as `i32`. We can use `Kilometers` anywhere we could use `i32`

We do not get the type-checking benefits that we get from the newtype pattern, that is, if we have:

```
type Kilometers = i32;
type Milometers = i32;
```

The compiler cannot protect us from passing `Milometers` to a function that is expecting `Kilometers` and vice-versa.

**Challenge**: redo the range_sum_bst leetcode challenge using a newtype and implementing `Deref`.

**Aside**: Creating a type that stores a closure:

```
type Thunk = Box<Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));
```

We can use aliases to reduce repetition in type declarations:

For example, in `std::io`:

```
type Result<T> = Result<T, std::io::Error>;
```

Since many methods in `std::io::Write` and other `struct`s in `std::io` always return `Result<T, std::io::Error>`, we can save some code by just having them return `Result<T>` which will expand to the type it aliases when compiled. We can also still use special syntax like the `?` operator since this is just an alias.

#### The Never Type that Never Returns

Rust has a special type called `!` that's known as the _empty type_ because it has no values. Rust refers to it as the _never type_ because it stands in place of the return type on functions that will never return.

```
fn bar() -> ! {
    // --snip--
}
```

The function `#bar` will never return, this is referred to as a _diverging function_.

```
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
}
```

Here, `continue` is a value of type `!`. Expressions of type `!` can be coerced into any other type.

Additionally, the `panic!` macro has type `!`. This allows us to use `panic!` as the return of a `match` arm

```
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

In the case of a neverending `loop`, `!` is the type of the expression:

```
println!("forever ");

loop {
    println!("and ever ");
}
```

_rust-analyzer_ will fill in the type annotation:;

```
let foo: ! = loop {
    println!("and ever ");
}
```

#### Dynamically Sized Types and the Sized Trait

_dynamically sized types_, _DSTs_, or _unsized types_ let us write code using values whose size cannot be known at compile time.

`str` is a _DST_, because we can't know how long the `str` is until runtime.

```
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

This can't compile because the compiler needs to know the size of a value at compile time, and all values of a given size must have the same type. Since `str`s of different length would need different amounts of storage, it is not possible to create a variable holding a _DST_.

The answer is to use `&str`. The slice data structure stores the starting position and length of the slice.

While `&T` is a single value that stores the starting position of a `T`, `&str` is _two_ values, the address and its length. Therefore `&str` has a size twice that of a `usize`. 

We can do this for any _DST_, we can put it behind pointer that knows the location and size of the dynamic information.

`Box<str>` or `Rc<str>` would work. We've also done this with trait objects, like `Box<Display>` or `&Display`.

_Every *trait* is a *dynamically sized type* we can refer to by using the name of the trait._

Rust uses the `Sized` trait to specify that a type's size is known at compile time. The trait is automatically derived for all types who's size is known at compile time.

Additionally, the compiler implicitly adds a bound on `Sized` to every generic function.

```
fn generic<T>(t: T) {
    // --snip--
}
```

becomes:

```
fn generic<T: Sized>(t: T) {
    // --snip--
}
```

Generic functions by default only work on types that have a size known at compile time. The restriction can be relaxed by with the following syntax:

```
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

This means "`T` can be size or not". This `?` syntax is only available for `Sized`. Also note the type parameter from `t` changed from `T` to `&T`. If `T` is not size, we can only refer to it via a pointer.

### Advanced Functions and Pointers

#### Function Pointers

We can pass regular functions as values just as we would closures. Functions coerce to the type `fn` (not `Fn`). `fn` is referred to as the _function pointer_ type.

```
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let anwser = do_twice(add_one, 5);
    println!("The answer is {}", answer);
}
```

We can also pass a closure in place of a `fn`:

```
let next_answer = do_twice(|x| x + 2, 5);
println!("The next answer is {}", next_answer);
```

However if we try to pass a closure that captures a variable: 

```
let foo = 24;
let last_answer = do_twice(|x| x + foo, 5);
println!("The next answer is {}", last_answer);
```

We get an error:

```
error[E0308]: mismatched types
   --> src/main.rs:394:32
    |
394 |     let last_answer = do_twice(|x| x + foo, 5);
    |                                ^^^^^^^^^^^ expected fn pointer, found closure
    |
    = note: expected fn pointer `fn(i32) -> i32`
                  found closure `[closure@src/main.rs:394:32: 394:43]`
note: closures can only be coerced to `fn` types if they do not capture any variables
   --> src/main.rs:394:40
    |
394 |     let last_answer = do_twice(|x| x + foo, 5);
    |                                        ^^^ `foo` captured here
```

If we try to pass a function to a function that is expecting a closure parameter, that will work just fine. Since functions never capture any variables from their environment, they will always satisfy `FnOnce`, `FnMut`, and `Fn`. see `chapter_19/src/main.rs#pass_a_function` for examples.

It is usually benficial to define functions to accept generic types with one of the closure traits. That way you can be more flexible and accept functions and closures that fit the trait bounds. If you were working with external code, such as C, where there are no closures, you would want to only accept an `fn`.

#### Returning Closures

Closures are represented by traits, which means you can't return closures directly. Usually if you wanted to return a trait you could instead use the concrete type that implements the trait as the return value. With closures, you can't do this because closures don't have a concrete type that is returnable.

If we try to return a closure with the trait bounds as the return type: 

```
fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}
```

We get a multiple errors:

```
error[E0782]: trait objects must include the `dyn` keyword
   --> src/main.rs:406:25
    |
406 | fn returns_closure() -> Fn(i32) -> i32 {
    |                         ^^^^^^^^^^^^^^
    |
help: add `dyn` keyword before this trait
    |
406 | fn returns_closure() -> dyn Fn(i32) -> i32 {
    |                         +++

error[E0746]: return type cannot have an unboxed trait object
   --> src/main.rs:406:25
    |
406 | fn returns_closure() -> Fn(i32) -> i32 {
    |                         ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Fn(i32) -> i32` as the return type, as all return paths are of type `[closure@src/main.rs:407:5: 407:14]`, which implements `Fn(i32) -> i32`
    |
406 | fn returns_closure() -> impl Fn(i32) -> i32 {
```

The answer is to `Box` the closure to create a trait object:

```
fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x + 1)
} 

fn main() {
  call_closure(returns_closure());
}
```

Notice that the Box will be `deref'd` automatically.

## Chapter 20 (Final Project: Building a Multithreaded Web Server)

### Building a Single Threaded Web Server

#### Listening for a Connection

_Hypertext Transfer Protocol_(HTTP), and _Transmission Control Protocol_(TCP). Both are _request-response_ protocols. The client initiates a request and a server listens and responds. TCP describes the details of how infomration gets from one server to another. HTTP defines the contents of the requests and responses. HTTP doesn't require TCP but it is almost always used with TCP.

```
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
```

`TcpListener#bind` creates a new instance of `TcpListener`. `listener#incoming` returns an iterator of `Result<TcpStream, Error>`s. A `TcpStream` reperesents an open connection between the client and the server, ie. the entire request response cycle where the client connects to the server, the server generates a response and closes the connection. `TcpStream` will both read from itself to see what the client sent and allow us to write a response to it. 

Note that we are iterating over connection attempts (`Result<TcpStream, Error>`) not actual connections, so we could receive errors if connections are not successful due to OS conditions.

When `stream` goes out of scope at the end of the loop, the implementation of `drop` on `TcpStream` closes the connection.

#### Reading the Request

```
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
```

`stream` is mutable here because the `TcpStream` type holds a pointer to the request data we can read from. To allow us to read the request data in segments, `TcpStream` may mutate this pointer.

`String::from_utf8_lossy` creates a `String` from a `&[u8]`. The `lossy` part means that if it encounters an invalid UTF-8 sequence, it will replace the invalid sequence with `�` (the U+FFFD replacement character).

#### A Closer Look at an HTTP Request

```
Request: GET / HTTP/1.1
Host: localhost:7878
Connection: keep-alive
Cache-Control: max-age=0
sec-ch-ua: " Not;A Brand";v="99", "Google Chrome";v="97", "Chromium";v="97"
sec-ch-ua-mobile: ?0
sec-ch-ua-platform: "macOS"
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange
```

The first line is the _request line_, which indicates the request _method_, followed by the _Uniform Resource Identifier_ (URI). The URI is followed by the HTTP version, and finally a _CRLF_ sequence. A _CRLF_ is a _carriage return_ and _line feed_ sequence, `\r\n`. The _CRLF_ sequence separates the request line from the rest of the request data.

The remaining lines in this request are all headers.

#### Writing a Response

Responses have the following format:

```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

The first line is a _status line_ which contains the HTTP version used in the response, a numeric status code and a reason phrase that provdides a text description of the status code.

Following the status line is the headers, and finally the body of the response.

```
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
```

`stream.flush()` will block the main thread until all the bytes are written to the connection. `TcpStream` contains an internal buffer to minimize calls to the underlying operating system.


#### Validating the Request and Selectively Responding

```
    let slash = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(slash) {
        //create response and write to stream
```

Here, because `buffer` is an array of raw bytes, we will compare it to a _byte string_ we created using the `b""` _byte_ string syntax.

### Turning our Single-Threaded Server into a Multi-threaded server

#### Improving Throughput with a Thread Pool

A _threadpool_ is a group of spawned threads that are waiting and ready to handle a task. When the program receives a new task, it assigns one of the threads in the pool to process the task.

Our _threadpool_ will be implemented with a request queue. The pool will maintain a queue of requests, each thread will dequeue a reqest, handle it, then ask the queue for another request.

**Big Challenge**: Instead of using a _threadpool_, implement the webserver with a _fork/join_ model, and a _async I/O_ model.

Results: _fork/join_ doesn't really make sense for a webserver. Find _async I/O_ example in `chapter_20/asyncio`

#### Compiler Driven Development

We're going to use an outside-in approach with _compiler driven development_ to create and make use of a `Threadpool` type.

```
fn main() {
    //--snip--

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
     //--snip--
}
```

**DELETE** This is where I'm going to start listing the steps in our workflow and describing how they link together like I did in chapter 12. 

After creating a `Threadpool` struct with a naive `new` associated method, the compiler tells us that `#execute` is not defined on `Threadpool`.

Before we can define `#exectute`, we need to decide what the signature will look like. We know we're planning to use it to pass a closure to `#spawn` that will call `handle_connection`. So it should accept a closure. The closure that `#spawn` accepts captures, then consumes (transfers ownership) of the `stream` to `handle_connection`, so we know `#execute` should take an `FnOnce` closure. 

We also know that `#execute` will be responsible for transferring the closure from our main thread to a thread waiting in the pool, so the closure argument to `#execute` should be `Send` as well.

Finally we do not know how long the thread will take to execute, (we can't relate the lifetime of the closure to any other references that are in scope) so we will give the closure the `'static` lifetime.

With these obeservations in consideration, we come up with a type signature for `#execute`:

```
impl ThreadPool {
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static 
    {
        //empty body
    }
}
```

If we compile this now we will get warnings for the unused variable `size` in `#new` and `f` in `#execute<F>`. Since the `size` value in `#new` is a `usize`, but we don't want zero threads, we need to validate that `size` is positive.

```
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }
```

 After we ensure that our `Threadpool` user is requesting a valid number of threads, we need to create the threads and store them somewhere so we can reference them when we're ready. `spawn` typically returns a `JoinHandle<T>` to allow us to reference a thread later, so we know we're going to need to store a collection of `JoinHandle<T>`s somewhere.

```
pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store their handles
        }

        ThreadPool {
            threads
        }
    }
}
```

Previously we have used `#spawn` create a thread and have it run a closure immediately. In the `for` loop in new, we would like to create threads and have them wait for `#execute` to activate them. The standard library provides no way to create a thread without it starting execution immediately, so we will have to implement it. We will create a new struct to represent waiting threads called `Worker`.

In `Worker::#new` we create a Worker that stores an `id` and a `thread` with an empty closure, just because `spawn` requires we give it _some_ closure.

**Question**: Does a thread with an empty closure not execute?

Now we will make the `Worker` structs fetch code to run from a queue held in the `ThreadPool` and send that code to its `thread` to run. Since we know we need to create a queue that other threads can read from, we should realize pretty quickly that we can implement this behavior with channels.

We will create a channel that the `ThreadPool` can send code (closures) down, and the `Worker`s can listen on. When a `Worker` receives a closure on the channel it executes it. /

First we create a channel and let the `ThreadPool` hold the sending end:

```
    let (sender, receiver) = mpsc::channel();

    // --snip--

    ThreadPool {
        workers,
        sender
    }
```

Then we want to send the receiving end of the channel to each worker.

```
   for i in 0..sizer {
       workers.push(Worker::new(id, receiver));
   }
```

```
impl Worker {
    fn new(id: usize, receiver: mpsc::Reciver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        })
    }
}
```

The next problem that arises is that when we are creating workers with `Worker.new()`, we are trying to use a moved value every time we pass `receiver`. The problem here is that we are trying to use `mpsc` (multiple producer, singler consumer) channels, but we are working with a _single producer, multiple consumer_ problem. The answer here is to share a single _receiver_ (queue) across all the workers. 

We are going to have to share a queue among all the workers. Additionally, taking a `Job` off the queue will require mutating the queue.

Since we know we need to share ownership of some data across multiple threads, we should be reaching for a tool to provide multiple ownership across threads, and another to allow for interior mutablility across threads. We should realize that `Arc<Mutex<T>>` is the answer.

**Question**: The book is going to have us store the receiving end of the `mpsc` channel in the `Arc<Mutex<T>>` so we can clone it and then pass it to each of the workers. This seems like an anti pattern. Why do we still need the mpsc channel? Why can't we just have the main thread add `Job`s to the queue held in the `Arc<Mutex<T>>` and then let the `Worker`s check the queue for `Job`s? 

Probable answer: There could be infinite `Jobs` created by the main thread, the `mpsc` channel is just a convenient queue implementation for us to use.

In `ThreadPool::new`:

```
    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    for id in 0..size {
        workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
```

In order to implement the execute method we will need to make Job a type alias for a _trait object_ that holds the type of closure that `execute` receives.

`type Job = Box<dyn FnOnce() -> () + Send + 'static>`

**Question**: Why are we able to pass a unboxed closure to `#execute` but we need to box it to send it on the channel?

Answer: If we try to do this, we will get an error:

```
error[E0308]: mismatched types
  --> src/lib.rs:35:26
   |
29 |     pub fn execute<F>(&self, f: F)
   |                    - this type parameter
...
35 |         self.sender.send(f).unwrap();
   |                          ^ expected struct `Box`, found type parameter `F`
   |
   = note:      expected struct `Box<(dyn FnOnce() + Send + 'static)>`
           found type parameter `F`
   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html
help: store this in the heap by calling `Box::new`
   |
35 |         self.sender.send(Box::new(f)).unwrap();
```

Although we would not be storing the closure `f` in any data structure, so it does not need to be boxed for satisfy the compiler's need for sized types, each thread will have a different call stack, so we need to box the closure so it ends up on the heap, where other threads can then access it.

#### Making Each Worker's Thread Execute Jobs.

```
let thread = thread::spawn(|| loop {
    let job = receiver.lock().unwrap().recv().unwrap();

    println!("Worker {} got a job; executing.", id);

    (*job)();
});
```

First we call `.lock` on the `Mutex` to request the `MutexGuard` (remember a `MutexGuard` is a smart pointer to the inner data). We have to unwrap the `MutexGuard` from the `Result<MutexGuard<T>, PoisonError<E>>` it is stored in. Then we can call receive on the channel which will block until a value is received, which will also come in a `Result<T,E>` that needs to be unwrapped.

We got another error regarding the closure capturing `receiver` and `id` via a borrow:

```
error[E0373]: closure may outlive the current function, but it borrows `receiver`, which is owned by the current function
  --> src/lib.rs:56:36
   |
56 |         let thread = thread::spawn(|| loop {
   |                                    ^^ may outlive borrowed value `receiver`
57 |             let job = receiver.lock().unwrap().recv().unwrap();
   |                       -------- `receiver` is borrowed here
```

We just need to indicate that the closure can take ownership of `receiver` and `id`. `id` will be used again late after the move, but that is okay because it is `Copy`.

### Graceful Shutdown and Cleanup

With our server working as intended, we still see 3 compiler warnings for unused variables, `workers` on `ThreadPool` plus `id` and `thread` on `Worker`. This indicates that we are not cleaning things up when our server shuts down.

We need to implement the `Drop` trait on `ThreadPool` and have it call `join` on each of the threads in the pool to guarantee each thread finishes serving its request before closing. We also need to tell each thread to stop accepting new requests.

#### Implementing the Drop Trait on `ThreadPool`

```
impl Drop for ThreadPool { fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
```

This first attempt results in an error: 

```
error[E0507]: cannot move out of `worker.thread` which is behind a mutable reference
  --> src/lib.rs:44:13
   |
44 |             worker.thread.join().unwrap();
   |             ^^^^^^^^^^^^^ move occurs because `worker.thread` has type `JoinHandle<()>`, which does not implement the `Copy` trait
```

`worker` is only a mutable reference, but `#join` takes ownership. We need to move the thread out of the `worker` instance so we can supply `#join` an owned value.

Since we are attempting to move a value out of a mutable reference, we should be reaching for the `#take` method!

First we'll have to wrap the `thread` in an `Option`.

```
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
```

Then we can use the `#take` method to replace the `Some(thread)` in `worker.thread` with a `None` value, while we capture the `thread` as an owned value:

```
fn drop(&mut self) {
    for worker in &mut self.workers {
        println!("Shutting down worker {}", worker.id);

        if let Some(thread) = worker.thread.take() {
            thread.join().unwrap();
        }
    }
}
```

#### Signaling to the Threads to Stop Listening for Jobs

We are now calling `join` on the all the worker threads in the `ThreadPool`, which will cause our main thread to wait for each to finish and then shut down. 

Unfortunately, each of the worker threads is still looping, attempting to acquire the Mutex to get more `Job`s. Given our current implementation, the main thread will wait forever for the first worker thread to finish.

To fix this, we need to modify the threads so they listen for _either_ a `Job` to run or a signal that they should stop listening an shut down.

Instead of `Job` instances, `#execute` will send one of two enum variants over the channel:

```
enum Message {
    NewJob(Job),
    Terminate
}
```

We have to modify `#execute` to send this new type on the channel, and also `#new`, to define the threads in a way that they unpack each `Message` using `match`. 

`#execute`:

```
pub fn execute<F>(&self, f: F)
where
    F: FnOnce() -> () + Send + 'static,
{
    let job = Box::new(f);

    self.sender.send(Message::NewJob(job)).unwrap();
}
```


`#new`:

```
pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || loop {
        let message = receiver.lock().unwrap().recv().unwrap();

        match message {
            NewJob(job) => {
                println!("Worker: {} got a job; executing.", id);

                job();
            },
            Terminate => {
                println!("Worker: {} was told to terminate.", id);

                break;
            },
        }
    });
    
    // --snip--
}
```

Now we are left with a warning because we are not creating any `Message`s of the `Terminate` variant. Recall that we were tying to tell our `Worker` threads to shut down in `ThreadPool::drop` dbefore we subsequently wait for the to finish shutting down. Now is the time to tell them to shut down:

```
fn drop(&mut self) {
    println!("Sending Terminate Message to all workers.");

    for _ in &mut self.workers {
        self.sender.send(Message::Terminate).unwrap();
    }

    for worker in &mut self.workers {
        println!("Shutting down worker {}", worker.id);

        if let Some(thread) = worker.thread.take() {
            thread.join().unwrap();
        }
    }
}
```

Notice that we need the two loops because the first worker to receive a `Terminate` `Message` on the channel may not be the first worker that we pull off the `Vec<Worker>` in `for worker in &mut self.workers`. This would mean that we would be using `#join` to wait for a thread to finish, but that thread will not have been sent `Message::Terminate` yet! This would result in a deadlock.

**Aside**: 

`#take` is defined on `Iterator` as well:

```
for stream in listener.incoming().take(2) {
    // --snip--
}
```

Here we see it limit the iteration of a `for` loop.

**Book Challenges**:

- Add more documentation to `TheadPool` and its public methods.
- Add tests of the library's functionality.
- Change calls to `unwrap()` to more robust error handling.
- Use `ThreadPool` to perform some task other than serving web requests.
- Find a thread pool create on _https://crates.io/_ and implement a similar web server using the create instead. Then compare the API and robustness to the threadpool we implemented.

### Creating an AsyncIO server.

From the _synchonous_ server: 

```
fn main() {
    // Listen for incoming TCP connections on localhost port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Block forever, handling each request that arrives at this IP address
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // --snipped--
}
```

We can modify `handle_connection` to return a type that implements `Future`:

async fn handle_connection(mut stream: TcpStream) {
    // --snipped--
}

A `Future` type represents a value that might not have finished computing yet.

```
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```
`poll` is a method does not block and tries to resolve to the final type. It is called repeatedly.

Often when it is implemented for a type, the associated type `Output` is left empty: `Future<Output=()>`. Which means `poll` will never return an output. 

Because we haven't `await`ed or `poll`ed `#handle_connection`, it will never run. So we modify our call to `handle_connection` to append an `await`:

```
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream).await;
    }
```

The compiler then tells us that since our program is currently synchronous, we need to use an asynchronous runtime to allow for the use of `await`.

```
error[E0752]: `main` function is not allowed to be `async`
 --> src/bin/main.rs:7:1
  |
7 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

#### Adding an async runtime

In `Cargo.toml`:

```
[dependencies]
tokio = { version = "1", features = ["full"] }
```

In `main.rs`:
```
#[tokio::main]
async fn main() {
```

The use of `std::thread::#sleep` in `handle_connection` will still cause `handle_connection` to block the main thread. We need to use the non-blocking version, `async_std::task::sleep` instead. 

```
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
    }
```

Above we also see that we are using the blocking `listener.incoming()` from `TcpListener`. We'll prefer to use the `tokio::net::TcpListener` instead:

```
#[tokio::main]
async fn main() {
    let listener: tokio::net::TcpListener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        handle_connection(stream).await;
    }
}
```

This will still cause the main thread to block. We can wrap `#handle_connection` in a green thread to process it asynchronously but still with one thread:

```
    tokio::spawn(async move {
        handle_connection(stream).await;
    });
```

## Apendix A: Keywords

Keywords cannot be used as names of functions, variables, parameters, struct fields, modules, crates, constants, macros, static values, attributes, types, traits or lifetimes.

Remember *Attributes*: are metadata about pieces of Rust code, such as `#[derive(_)]` or `#[test]`.

*as*: Perform primitive casting, disambiguate the specific trait containing an item, or rename items in `use` and `extern crate` statements.

Cast between types:
    `let thing1: u8 = 89.0 as u8;`
Rename an import:
    `use std::{mem as memory, net as network};`
    `extern crate foo as bar;`
Disambiguate the specific trait containing an item:
    `println!("A baby dog is called a {}", <Dog as Animal>::baby_name());`

*break*: exit a loop immediately

*const*: define constant items or constant raw pointers
    
```
const DAYS: &[&str] = &[
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh"
];
```

raw pointers:

```
let mut num = 5;
let r1 = &num as *const i32;
let r2= *mut num as *mut i32;
```

*continue*: continue to the next loop iteration

*crate*: link an external crate or a macro variable representing the crate in which the macro is defined

```
extern crate add_one;
```

**Challenge**: figure out what a "macro variable" is and how it can reprensent where a crate is defined. 

*else*

*enum*

*extern*: Link an external crate, function or variable.

Make your Rust create aware of other crates within your project:

```
`extern crate foo;`
```

Decalare function interface so rust can call foreign code using _FFI_:

```
#[link(name = "my_c_library")]
extern "C" {
    fn my_c_function(x: i32) -> bool;
}
```

Define a function callable from other languages using _FFI_:

```
#[no_mangle]
pub extern "C" fn callable_from_c(x: i32) -> bool {
    x % 3 == 0
}
```

*false*

*fn*

*for*

loop over items from an iterator or range:

```
for i in 0..size {
    // --snip--     
}
```

implement a trait:

```
impl Debug for MyRectangle {
    // --snip--
};
```

Specify a higher-ranked lifetime:

```
where for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
```
(Where Fn(a, b, c) -> d is itself just sugar for the unstable real Fn trait)

`for<'a>` can be read as "for all choices of `'a`", and basically produces an infinite list of trait bounds that `F` must satisfy. Intense. There aren't many places outside of the Fn traits where we encounter HRTBs, and even for those we have a nice magic sugar for the common cases.

**Challenge**: consider revisiting this someday. Use the example from the [Rustonomicon](https://doc.rust-lang.org/nomicon/hrtb.html)

*if*

*impl*: implement inherent or trait functionality

*in*: part of `for` loop syntax.

*let*: bind a variable

*loop*: loop unconditionally

*match*: match a value to patterns

*mod*: define a module

*move*: make a closure take ownership of all its captures.

*mut*: denote mutability in references, raw pointers, or pattern bindings

In references and raw pointers:

```
    let mut x = "foobar".to_string();
    let mutable = &mut x;
    let raw = mutable as *mut String;
```

In pattern bindings: 

```
let mut robot_name = Some("borg".to_string());

match robot_name {
    Some(ref mut name) => *name = "blork".to_string(),
    None => (),
}
```

Remember using `Some(&mut name)` would try to match an existing reference and bind `name` to an owned value. This in practice would only work with values that are `Copy`.

*pub*: denote public visibility in struct fields, impl blocks, or modules

*ref*: bind by reference (create a reference to owned data through destructuring, see above)

*return*

*Self*: a type alias for the type implementing a trait

*self*: method subject or current module

```
impl Pokemon for Dog {
    pub fn evolve(&self) {
        self.form += 1
    }
}
```

In paths, self can be used to refer to the current module, either in a use statement or in a path to access an element:

```
use std::io::{self, Read};
```

Is functionally the same as:

```
use std::io;
use std::io::Read;
```

*static*: global variable or lifetime lasting the entire program execution

global:

```
static HELLO_WORLD &str = "Hello, World!";
```

lifetime: 

```
let s: &'static str = "I have a static lifetime.";
```

*struct*

*super*: parent module of the current module

```
use super::*;
```

*trait*: define a trait

*true*

*type*: define a type alias or associated type

Associated type:

```
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
    }
}
```

Type alias (**Challenge** try implementing this including an implementation for `Deref`):

```
type Tree Rc<RefCell<TreeNode>>;
```
*union*: define a union and is only a keyword when used in a union declaration 

A union looks like a struct but just allows for the interpretation of a single piece of data as multiple types:

```
    union IntOrFloat {
        i: u32,
        f: f32,
    }

    let mut u = IntOrFloat { f: 1.0 };

    // Reading the fields of a union is always unsafe
    assert_eq!(unsafe { u.i }, 1065353216);

    // Updating through any of the field will modify all of them
    u.i = 1073741824;
    assert_eq!(unsafe { u.f }, 2.0);
```

Matching on a union works too:

```
    let u = IntOrFloat { f: 11.0 };

    unsafe {
        match u {
            IntOrFloat { f: 10.0 } => println!("Found ten!"),
            IntOrFloat { i } => println!("Found an int: {}", i),
        }
    }
```

*unsafe*: denote unsafe code, functions, traits or implementations

```
unsafe trait Foo {

}

unsafe impl Foo for i32 {

}
```

*use*: import symbols into scope*

*where*: denote clauses that constrain a type

```
pub fn notify<T>(item: T)
where
    T: Summary
```

*while*: loop conditionally based on the result of an expression

```
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

## Appendix B: Operators and Symbols

Operators and other symbols appear by themselvs or in the context of paths, generics, trait bounds, macros, attributes, comments, tuples and brackets.

### Operators

| Operator | Example                            | Explanation                         | Overloadable? |
| -------- | ---------------------------------- | ----------------------------------- | ------------- |
| !        | ident!(..), ident!(..), ident!(..) | Macro Expansion                     |               | <-- what                         |
| !        | !expr                              | Bitwise or logical complement       | Not           |
| !=       | var != expr                        | Nonequality comparison              | PartialEq     |
| %        | expr % expr                        | Arithmetic Remainder                | Rem           |
| %        | var %= expr                        | Arithmetic Remainder and assignment | RemAssign     |
| &        | &expr, &mut expr                   | Borrow                              |               |
| &        | &type                              | Borrowed pointer type               |               |
| &        | expr & expr                        | Bitwise AND                         | BitAnd        |
| &=       | var &= expr                        | Bitwise AND and assignment          | BitAndAssign  |
| &&       | expr && expr                       | short-circuting logical AND         |               |
| *        | expr * expr                        | Arithmentic multiplication          | Mul           |
| *=       | var *= expr                        | multiplication and assignment       | MulAssign     |
| *        | *expr                              | Dereference                         | Deref         |
| *        | &const type, &mut type             | Raw pointer                         |               |
| +        | impl<T: 'a Display + PartialOrd>   | Compound type constraint            |               |
| +        | expr + expr                        | Arithmetic addition                 | Add           |
| +=       | var += expr                        | Arithmetic addition and assign      | Addassign     |
| ,        | expr, expr                         | Argument and element separater      |               |
| -        | - expr                             | Arithmetic negation                 | Neg           |
| -        | expr - expr                        | Arithmetic subtraction              | Sub           |
| -=       | var -= expr                        | Subtraction and assignment          | SubAssign     |
| ->       | where T: Fn(u32 -> u32),           |                                     | -> u32        | Function and closure return type |             |
| .        | expr.ident                         | Member access                       |               |
| ..       | .., expr.., ..expr, expr..expr     | exclusive range literal             | PartialOrd    |
| ..=      | ..=expr, expr..=expr               | inclusive range literal             | PartialOrd    |
| ..       | Rectangle { x1: 5, ..rectangle }   | Struct literal update syntax        |               |
| ..       | variant(x, ..), Rectangle { x, ..} | "And the rest" pattern binding      |               |
| /        | expr / expr                        | Arithmetic division                 | Div           |
| /=       | var /= expr                        | Arithmetic division and assignment  | DivAssign     |
| :        | pat: type, ident: type             | Constraints such as arguments       |               |
| :        | ident: expr                        | Struct field initializer            |               |
| :        | 'a: loop {                         | Loop label                          |               | <-- what                         |
| ;        | expr;                              | Statement and item terminator       |               |
| ;        | [...; len]                         | Part of fixed size array syntax     |               |
| <<       | expr << expr                       | Left-shift                          | Shl           |
| <<=      | var <<= expr                       | Left-shift and assignment           | ShlAssign     |
| <        | expr < expr                        | Less than comparison                | PartialOrd    |
| =        | var = expr, ident = type           | Assignment/Equivalence              |               |
| ==       | expr == expr                       | Equality comparison                 | PartialEq     |
| =>       | pat => expr                        | Part of match arm syntax            |               |
| >        | expr > expr                        | Greater than comparison             | PartialOrd    |
| >=       | expr >= expr                       | Greater than or equal to comparison | PartialOrd    |
| >>       | expr >> expr                       | Right-shift                         | Shr           |
| >>=      | var >>= expr                       | Right-shift and assignment          | ShrAssign     |
| @        | User { id: my_id @ 3..=7 } => prin | Pattern binding                     |               |
| ^        | expr & expr                        | Bitwise exclusive OR                | BitXor        |
| ^=       | var ^= var                         | Bitwise exclusive OR and assignment | BitXorAssign  |
|          |                                    | pat                                 | pat           | Pattern alternatives             |             |
|          |                                    | expr                                | expr          | Bitwise OR                       | BitOr       |
|          | =                                  | var                                 | = expr        | Biwise OR and assignment         | BitOrAssign |
|          |                                    |                                     | expr          |                                  | expr        | Short=circuting logical OR |  |
| ?        | expr?                              | Error propagation                   |               |

### Symbols

Symbols don't behave like function or method calls, so they are not operators, their usage is still reserved

#### Symbols that appear on their own

| Symbol/Example                 | Explanation                                                          |
| ------------------------------ | -------------------------------------------------------------------- |
| 'ident                         | Named lifetime or loop lable                                         | <-- what |
| ...u8, ..usize, etc            | Numeric literal of specific type                                     |
| "foobar"                       | String literal                                                       |
| r"foo", r#"f"o", r##"f#o"##    | Raw string literal, escape characters not processed                  |
| b"foo"                         | Byte string literal, constructs a [u8] instead of str                |
| br"foo", br#"f"o", br##"f#o"## | Raw bytes string literal                                             |
| 'f'                            | Character literal                                                    |
| b'f'                           | ASCII byte literal                                                   |
|                                | _                                                                    | expr     | Closure |
| !                              | Always empty bottom type for diverging functions                     |
| 1_000_000, _ => printlne!()    | ignored pattern binding; also used to make integer literals readable |


#### Symbols that appear in the context of a path

| Symbol                              | Example                        | Explanation                                     |
| ----------------------------------- | ------------------------------ | ----------------------------------------------- |
| ident::ident                        | std::env::Args                 | Namespace path                                  |
| ::path                              | ::outermost::secret_function() | Path relative to the create route               |
| self::path                          |                                | Path relative to the current module             |
| super::path                         |                                | Path relative to tthe parent module             |
| type::ident, <type as trait>::ident | String::from("foo")            | Associated constants, functions, and types      |
| <type>::...                         | <&T>::..., <[T]>::...          | Associated item for a type that cannot be named |
| trait::method(...)                  |                                | Disambiguate method call by specifying trait    |
| type::method(...)                   |                                | Disambiguate method call by specifying type     |
| <type as trait>::method             |                                | Disambiguate method call by naming trait & type |

#### Symbols that appear in the context of generic type paramters

| Symbol                     | Example                          | Explanation                                   |
| -------------------------- | -------------------------------- | --------------------------------------------- |
| path<...>                  | Vec<u8>                          | Specifies paramters to generic type in a type |
| path::<...>, method::<...> | "42".parse::<i32>()              | Turbofish                                     |
| fn ident<...>  ...         | fn my_func<T>() {}               | Define generic function                       |
| struct ident<...>          | struct my_type<T>                | Define generic structure                      |
| enum ident<...>            | enum my_type<T>                  | Define generic enumeration                    |
| impl<...> ...              | impl<T> Display for Rectangle<T> | Define generic implementation                 |
| for<...> type              |                                  | Higher-ranked lifetime bounds                 |
| type<ident=type>           | Iterator<item=i32>               | Generic type with specified associated type   |

#### Symbols that appear in the context of constraining generic type parameters with trait bounds

| Symbol                | Explanation                                                                        |
| --------------------- | ---------------------------------------------------------------------------------- |
| T: U                  | Generic parameter T constrained to types that implement U                          |
| T: 'a                 | Generic type T must outlive lifetime 'a. It can't contain any refs shorter than 'a |
| T: 'static            | Generic type T contains no borrowed references other than 'static ones             |
| 'b: 'a                | Generic lifetime 'b must outlive lifetime 'a                                       |
| T: ?Sized             | Allow generic type parameter to by dynamically sized type                          |
| 'a + U, trait + trait | Compound type constraint: generic paramter T outlives 'a and implements U          |

#### Symbols that appear in the context of calling or defining macros and specifying attributes on an item

| Symbol                                | Explanation        |
| ------------------------------------- | ------------------ |
| #[meta]                               | Outer attribute    |
| #![meta]                              | inner attribute    |
| $ident                                | Macro substitution | <-- what |
| $ident:kind                           | Macro capture      | <-- what |
| $(...)...                             | Macro repetition   | <-- what |
| ident!(...), ident!{...}, ident![...] | Macro invocation   | <-- what |

#### Symbols that create comments

| Symbol   | Explanation                                                                      |
| -------- | -------------------------------------------------------------------------------- |
| //       | Line comment                                                                     |
| //!      | Inner line doc comment (applies to the parent, usually used to document modules) |
| ///      | Outer line doc comment                                                           |
| /*...*/  | Block comment                                                                    |
| /*!...*/ | Inner block doc comment (applies to the parent)                                  |
| /**...*/ | Outer block doc comment                                                          |

#### Symbols that appear in the context of using tuples

| Symbol          | Explanation                                                                      |
| --------------- | -------------------------------------------------------------------------------- |
| ()              | Empty (unit) tuple literal and type                                              |
| (expr)          | Parenthesized expression                                                         |
| (expr,)         | Single-element tuple expression                                                  |
| (type,)         | Single-element tuple type                                                        |
| (expr, ...)     | Tuple expression                                                                 |
| (type, ...)     | Tuple type                                                                       |
| expr(expr, ...) | Function call expression, initialize tuple struct, initialize tuple enum variant |
| expr.0, expr.1  | Tuple indexing                                                                   |

#### Square Brackets

| Symbol                                     | Explanation                                                                                                          |
| ------------------------------------------ | -------------------------------------------------------------------------------------------------------------------- |
| [...]                                      | Array literal                                                                                                        |
| [expr; len]                                | Array literal containing len copies of expr                                                                          |
| [type; len], `let x: [i32; 5] = [4; 5];`   | Array type containing len instance of type                                                                           |
| expr[expr]                                 | Collection indexing. Overloadable (Index, IndexMut)                                                                  |
| expr[..], expr[a..], expr[..b], expr[a..b] | Collection indexing pretending to be collection slicing, using Range, RangeFrom, RangeTo or RangeFull as the "index" |

**Question** What does this mean when it says "Collection indexing _pretending_ to be collection slicing"?

Possible answer: it seems this is just refering to Rust's facility for creating slices. It could have just said "Collection indexing used to create slices from collections."

## Appendix C: Derivable Traits

Libraries can implement `derive` for their own traits, making the list of traits you can use `derive` with truly open-ended. Implementing `derive` involves using a procedural macro.  

### Partial Eq and Eq

`PartialEq`: Says two structs are equal if all their fields are equal, and that `enum` variants are equal to themselves. Allows us to use `assert_eq!`, `==`, etc...

`Eq`: Has no methods, just marks that for every value of a type, the value is equal to itself. Only types that can implement `PartialEq` can implement `Eq`, but the converse is not true. Implementing `Eq` allows us to use a type as a key for a `HashMap`, marking a type `Eq` tells the compiler that it will always be able to tell if two keys of the type are the same. Note this is why `f32`, for example, cannot be used for `HashMap` keys, because the `NaN` value is not equal to itself.

Also note that the reason `PartialEq` is named such is because it is not valid for all values in the range.

### PartialOrd and Ord

`PartialOrd`: Allows comparison of a type for sorting purposes. A type that implements `PartialOrd` allows for the use of `<, >, <=, and =>`.

Deriving `PartialOrd` implements the `partial_cmp` method which returns an `Option<Ordering>` that will be `None` when the values don't produce an ordering (such as the `NaN` `f32` value). 

When derived on `structs`, `PartialOrd` compares two instances by comparing the value in each field in the order in which the fields appear in the struct definition. When derived on `enum`s, variants declared earlier in the `enum` definition are considered less than those defined later. 

The `Ord` trait marks a type to say that for any two values o the type, an ordering exists. The `Ord` trait implements `#cmp` which returns an `Ordering` rather than `Option<Ordering>`. `Ord` can be derived on types that implement `PartialOrd` and `Eq`. That is when the values don't produce an ordering, they can be considered equal.

`Ord` is required to store a type `T` in a `BTreeSet<T>`, which stores data based on the sort order of the values.

### Clone and Copy for Duplicating Values

`Clone` implementations allow us to run arbitrary code in order to copy (deep copy) heap data. We can derive `Clone` if all fields or values in a given type also implement `Clone`.

`Clone` is often required to use methods that construct different types, such as to use `#to_vec` on a slice to construct a `Vec`. To use `to_vec` to create a `Vec<T>`, `T` must be `Clone`.

`Copy` allows you to duplicate a value only by copying bits stored on the stack. No arbitrary code is necessary (or allowed, as `Copy` doesn't expose any methods). `Copy` can also be derived when all fields or values of a given type also implement `Copy`. Any type that is `Copy` is also `Clone`. Everything possible with `Copy` is also possible with `Clone`, it may just take longer.

We've seen many examples of when `Copy` allows a value to be copied implicitly, avoiding the need to borrow values that are owned.

### Hash for Mapping a Value to a Value of Fixed Size

Types that implement `Hash` have a `hash` function which allows you to take an instance of a type of arbitrary size and map it to a value of fixed size. Deriving `Hash` is again possible if all the values/fields of a given type implement `Hash`. The derived `hash` method implementation combines the result of calling `hash` on each of the values/fields of the type.

### `Default` for Default Values

The `Default` trait allows you to create a default value for a type. Deriving `Default` implements the `default` function. The derived `default` function calls the `default` function on each part of the type (which also must implement `Default`).


`Default::default` is often used with struct update syntax:

```
    let foo = Foo {
        x: 1,
        ..Default::default()
    };
```

`Default` is required when you call `unwrap_or_default` on `Option<T>`, which returns the result of `Default::default` for the type `T` when the `Option<T>` is `None`.

### New Version Appendix D - Useful Development Tools

#### Automatic Formatting with `rustfmt`

`rustfmt`, which can be run with either `cargo fmt`, `cargo-fmt`, or `rustfmt` if you provide file paths, formats code for style.

**Challenge**: Get vs code or your current editor to automatically run `cargo fmt` when a rust file is saved.

Answer: In `settings.json`:

```
    "[rust]": {
        "editor.formatOnSave": true,
    },
```

#### Fix Your Code with `rustfix`

We can run `rustfix` by calling `cargo fix`, which will fix certain compiler warnings.

#### More Lints with Clippy

`clippy` provides more lints to catch common mistakes. 

**Challenge**: Consider running clippy either before commit, push, or merge. 

### Appendix D Macros

#### The 4 types of Macros in Rust

**Macros** are loosely defined as Rust code that writes other Rust code. There are 4 types of macros in rust, one is _declarative_, and is defined using `macro_rules!`, and the remaining three are _procedural_: 

- Custom `#[derive]` macros that specify code added with the `derive` attribute on a given type. Ex: `#[derive(debug)]`
- Attribute-like macros that define custom attributes usable on any item. Ex: 
- Function-like macros that look like function calls but operate on the tokens specified as their argument.

Macros don't need to have a finite list of paramters, and are expanded at the beginning of compilation. So we can use a macro to do things that functions cannot, like implement a trait.

Macros also must be defined before their usage.

#### Declarative Macros with `macro_rules!`

During compilation, `macro_rules!` macros take a piece of source code, match it against a set of patterns, and then replace it with a new piece of source code based on which pattern it matched.

##### An example: `vec!`

We can use `vec!` to create a vector of arbitrary lenght or type, which wouldn't work with a function because we couldn't give it a type before compilation. 

```
#[macro_export]
macro_rules! vec {
    ( $( $x:expr),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

`#[macro_export]` just makes this macro available whenver the enclosing crate is brought into scope.

Notice that the macro definition resembles a `match` expression with the `<pattern> => <expression>` syntax.

##### Macro Pattern Syntax

The syntax for matching patterns in macros differs from that of regular `match` expressions since we are matching Rust code instead of values. The full documentation of macro pattern syntax can be found in the [reference](https://doc.rust-lang.org/reference/macros-by-example.html).

Breaking down the above pattern, `( $( $x:expr),* )`

The outermost set of `()` just specifies that this is a macro pattern, which leaves us with :`$( $x:expr),*`

The `$()` captures values that match the pattern for use in the replacement code.

`$x:expr`, Matches any Rust expression, and names it `$x`.

The `,` after `$( $x:expr)` indicates that the pattern can also match code that has a literal `,` after the code that matches the code in the `$( $x:expr)`. 

Finally the `*` specifies that the pattern matches zero or more of whatever precedes the `*`. So in this case, `,*` means that the pattern can match an expression with zero, or more commas.

When taken together `( $( $x:expr),* )` simply means we will match any code that is a comma separated block of expressions of any length, and repeately bind those expressions to the variable `$x` for use in the match arm.

#### Pattern within the match arm

```
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
```

`temp_vec.push()` within `$()*` is generated for each part that matches `$()` in the pattern zero or more times, the `$x` is replaced with the expression matched.

So basically we are saying with the first `$()`, "match a thing", the second `$()` says "generate a thing", and the `*` says do it zero or more times depending on how many times as the pattern matches.

When this macro is compiled, the following transformation will happen:

```
vec![1,2,3]
```
   ||
   ||
   ||
  \  /
   \/

```
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

More information on declarative macros can be found in ["The Little Book of Rust Macros"](https://veykril.github.io/tlborm/)

**Question**: What happened to the `[]` in `vec![1,2,3]`? Are they just ignored by the pattern and forgotten?

_Answer_: From ["The Little Book of Rust Macros"](https://veykril.github.io/tlborm/):

Each "rule" looks like so:

```
    ($pattern) => {$expansion}
```

Actually, the parens and braces can be any kind of group, but parens around the pattern and braces around the expansion are somewhat conventional.

So we can use `vec!(1,2,3)` and `vec!{1,2,3}` in addition to `vec![1,2,3]`.

### Procedural Code for Generating Code from Attributes

Procedural macros accept some code as input, operate on that code, and produce some code as an output rather than matching against pattern and replacing the code with other code as decalarative macros do.

The three kinds of procedural macros (custom derive, attribute-like, and function-like) all work in a similar fashion.

Procedural macro definitions must reside in their own crate with a special crate type.

### Writing a Custom `derive` macro

If we want to create a trait that will allow a type to report its name, we implement it with an default implementation, since a running rust program has no information about types (including type names).

We'll need a macro to genearte the trait implementation at compile time.

As we said before, procedural macros need to be in their own crate. The convention for structuring crates and macro crates is as follows: for a crate named foo, a custom derive procedural macro crate is called foo_derive.

So inside our crate `hello_macro` that houses the trait `HelloMacro`, we'll create another library crate called `hello_macro_derive`.

Even though we have created our `hello_macro_derive` crate inside our `hello_macro` crate, the two still need to be published separately, and developers will need to bring them both into scope separately. We could also have `hello_macro` `use hello_macro_derive;`, and then re-export `hello_macro_derive`, but we keep them separate in this case.

**Challenge**: `use hello_macro_derive;` in `hello_macro`, and then re-export the macro for use in other crates.

#### Declaring our `hello_macro_derive` crate to be a procedural macro crate

in `apendix_D/hello_macro/hello_macro_derive/Cargo.toml`:

```
[lib]
proc-macro = true
```

in `apendix_D/hello_macro/hello_macro_derive/src/lib.rs`:

```
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}
```

The `proc_macro` crate is the compiler’s API that allows us to read and manipulate Rust code from our code.

The `syn` crate parses Rust code from a string into a data structure that we can perform operations on.

The `quote` crate turns `syn` data structures back into Rust code. 

`#[proc_macro_derive(HelloMacro)]` will allow a user to specify `#[derive(HelloMacro)]` for their types.

When we use `syn::parse` to parse the input stream, we'll get a structure that looks like this:

```
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

We can see that `ast.ident.ident` will hold the name of the type.

```
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!(
                    "Hello, Macro! My name is {}!",
                    stringify!(#name)
                );
            }
        }
    }
    gen.into()
}
```

The `quote!` macro lets us define the Rust code we want to return. `into()` converts the intermediate representation into a `TokenStream`.

The `#` in `#name` is a templating feature provided by  `quote!`. See the [quote crate](https://docs.rs/quote) for more information.

`stringify` will take an expression and turn it into a string literal without evaluating it. `stringify(1 + 2)` will evaluate to `"1 + 2"`. This will also happen at compile time.

Taking this all together, `impl_hello_macro` should take an ast representing the input type, and return a `TokenStream` of the code needed to implement `HelloMacro` for that type.

If we add a crate with dependencies hello_macro and hello_macro_derive and then `use   hello_macro::HelloMacro;` and `use hello_macro_derive::HelloMacro;`, we can `#[derive(HelloMacro)]` for any type we define to get the functionality described by the `HelloMacro` trait.

### Attribute-like Macros

Similar to _derive macros_, _attribute-like macros_ allow you to create new attributes. _Attribute-like macros_ can also be applied to other items, such as functions in addition to structs and enums.

See `apendix_D/attribute_like/src/main.rs` for an example of using a `route` attribute that annotates functions for use in a web framework.

**Challenge**: Implement a crate with the proc-macro crate type and implement an attribute that generates the code you want!

### Function-like Macros

_Function-like_ Macros define macros that look like function calls. Just like `macro_rules!` macros, they can take an unknown number of arguments. The difference is `macro_rules!` macros can only be defined using the match-like syntax.

Much like custom `derive` macros and _attribute_ defining macros, _function-like_ macros take `TokenStream` as input, and like the other procedural macros manipulate that `TokenStream` to return a `TokenStream` representing the code we want to generate.

```
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {}

fn main() {
    let sql = sql!(SELECT * FROM posts WHERE id=1);
}
```

**Challenge**: Figure out if _function-like_ macros need to be encapsulated in a `proc-macro` type crate, then implement a _function-like_ macro that generates the code you want!

### (Unrelated: Referring to type at runtime)

Note that when we refer to a unit struct in our program, we are actually instantiating a value of that type:

```
struct foobar;

fn main() {
    let x = foobar;
    App.new().servers(x).bind()
}
```

There is no way to refer directly to a type by name at runtime, since that information does not exist at runtime. So if we check the type of the value...

```
struct foobar;

fn main() {
    let x = foobar;
    App.new().servers(x).bind()
    let () = x;
}
```

We will see that it has been instantiated as a struct:

```
let () = x;
    ^^   - this expression has type `foobar`
    |
    expected struct `foobar`, found `()`
```

Also, If we try to refer to a struct that holds data in this way...

```
struct foobar {
    x: i32,
    y: i32,
};

fn main() {
    let x = foobar;
}
```

we will also get an error:

```
let x = foobar;
        ^^^^^^ help: use struct literal syntax instead: `foobar { x: val, y: val }`
```