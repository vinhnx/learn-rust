# learn-rust

I'm learning Rust from [examples](https://doc.rust-lang.org/rust-by-example/).

Goal is trying to get to know the syntax, concept, paradigm...

Also trying to solve every example's `Activity` challenge by the end of each section.

I am very fascinated by [their ecosystem and tooling](https://github.com/topics/rust).

Hope to build some tool by the end of it. :rocket:

# Learning NOTE

Rust has no `return` keyword

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
