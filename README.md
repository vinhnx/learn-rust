# learn-rust

I'm learning Rust from [examples](https://doc.rust-lang.org/rust-by-example/).

Goal is trying to get to know the syntax, concept, paradigm...

Also trying to solve every example's `Activity` challenge by the end of each section.

I am very fascinated by [their ecosystem and tooling](https://github.com/topics/rust).

Hope to build some tool by the end of it. :rocket:

# Learning NOTE

A note on printing performance
Printing to the terminal is surprisingly slow! If you call things like println! in a loop, it can easily become a bottleneck in an otherwise fast program

Printing errors
Printing errors should be done via stderr to make it easier for users and other tools to pipe their outputs to files or more tools.

==
Rust's Result<Ok,Err>

==

```rust
fn from_args() -> Self
where
    Self: Sized,
```

Gets the struct from the command line arguments. Print the error message and quit the program in case of failure.


==
Parsing command line arguments
A typical invocation of our CLI tool will look like this:

```bash
$ grrs foobar test.txt
```

We expect our program to look at test.txt and print out the lines that contain foobar. But how do we get these two values?

The text after the name of the program is often called the “command line arguments”, or “command line flags” (especially when they look like --this). Internally, the operating system usually represents them as a list of strings – roughly speaking, they get separated by spaces.

==

'a' : char 
"a": string

==

casting

==

Rust's pattern matching keyword is `match`
(`switch case` in Swift)

==

`rustc` stands for rust compiler

==

"The same year, work shifted from the initial compiler (written in OCaml) to the self-hosting compiler written in Rust.[37] Named rustc, it successfully compiled itself in 2011.[38] rustc uses LLVM as its back end."

==

"Rust features type inference, for variables declared with the let keyword. Such variables do not require a value to be initially assigned to determine their type. A compile time error results if any branch of code fails to assign a value to the variable.[33] Variables assigned multiple times must be marked with the mut keyword."

==

"Rust supports interface inheritance, but replaces implementation inheritance with composition; see composition over inheritance."

==

"The type system supports a mechanism similar to type classes, called "traits", inspired directly by the Haskell language. This is a facility for ad hoc polymorphism, achieved by adding constraints to type variable declarations"

==

"The concrete syntax of Rust is similar to C and C++, with blocks of code delimited by curly brackets, and control flow keywords such as if, else, while, and for. Not all C or C++ keywords are implemented, however, and some Rust functions (such as the use of the keyword match for pattern matching) will be less familiar to those versed in these languages. Despite the superficial resemblance to C and C++, the syntax of Rust in a deeper sense is closer to that of the ML family of languages as well as the Haskell language. Nearly every part of a function body is an expression,[26] even control flow operators. For example, the ordinary if expression also takes the place of C's ternary conditional. A function need not end with a return expression: in this case the last expression in the function creates the return value."

==

"To replicate the function in other languages of pointers being either valid or NULL, such as in linked list or binary tree data structures, the Rust core library provides an option type, which can be used to test if a pointer has Some value or None."

==

"Values can be passed by immutable reference using `&T`, by mutable reference using `&mut T` or by value using `T`"


==

"A function need not end with a return expression: in this case the last expression in the function creates the return value."

Rust has `return` keyword, but it is customary to omit it

use omit semicolon instead

Note: when the last line of a `fn` has no semicolon `;`

it is returning function

```rust
fn foo() -> fmt::Result {
    write!("done") // notice no semicolon here
}
```

==

`&` == mean reference to an object

https://stackoverflow.com/questions/31908636/what-does-the-ampersand-mean-in-a-rust-type

==

> https://doc.rust-lang.org/std/fmt/
> Formatting traits
> When requesting that an argument be formatted with a particular type, you are actually requesting that an argument ascribes to a particular trait. This allows multiple actual types to be formatted via {:x} (like i8 as well as isize). The current mapping of types to traits is:
* nothing ⇒ Display
* ? ⇒ Debug
* x? ⇒ Debug with lower-case hexadecimal integers
* X? ⇒ Debug with upper-case hexadecimal integers
* o ⇒ Octal
* x ⇒ LowerHex
* X ⇒ UpperHex
* p ⇒ Pointer
* b ⇒ Binary
* e ⇒ LowerExp
* E ⇒ UpperExp

==

prefer snake_case to camelCase

like Python

==

Rust: trait (protocol in Swift?)

```rust
fmt::Display: {}
fmt::Debug: {:?}
```

==

note to self: 

on VSCode, can hover on `Display` and copy the code no need to memorized it
￼
==

TIL: Swift's Result is inspired by Rust
http://doc.rust-lang.org/1.33.0/alloc/fmt/type.Result.html

```rust
type Result = Result<(), Error>;
```

==

what does `<'a>` or `&'a` means in Rust?

https://stackoverflow.com/questions/47640550/what-is-a-in-rust-language

generic?

==

Guides:

https://doc.rust-lang.org/rust-by-example

https://doc.rust-lang.org/book/ch00-00-introduction.html

https://github.com/rust-lang/rustlings

==

> Printing is handled by a series of macros defined in std::fmt some of which include:
* format!: write formatted text to String
* print!: same as format! but the text is printed to the console (io::stdout).
* println!: same as print! but a newline is appended.
* eprint!: same as format! but the text is printed to the standard error (io::stderr).
* eprintln!: same as eprint!but a newline is appended.
https://doc.rust-lang.org/rust-by-example/hello/print.html
