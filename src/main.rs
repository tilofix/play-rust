// -*- compile-command: "cargo +nightly rustc -- -Z unpretty=expanded" -*-
// Required to generate "run" button
#![doc(html_playground_url = "https://play.rust-lang.org/")]

//! # Crate to learn hello world
//!
/*!
Let's see where this block documentation comment `/*! ... */` goes ...
look!, it's an inner comment and applies to the parent which is the crate.

By the way - to have the link available - `rustdoc` uses the
[CommonMark Markdown specification](https://commonmark.org/).

See!, for documentation comments the same notation is applied
to distiguish between [inner and outer comments](https://doc.rust-lang.org/reference/comments.html)
as for [inner and outer attributes](https://doc.rust-lang.org/reference/attributes.html), respectivily:
- __Inner attributes/doc comments__, written with a bang (`!`) after the hash (`#`),
  apply to the item that the attribute is declared within, apply to the parent of the comment/attribute.
- __Outer attributes/doc comments__, written without the bang after the hash,
apply to the thing that follows the attribute.

|       | attributes | line comments | block comments |
|-------|------------|---------------|----------------|
| inner | `#![ATTR]` | `//!`         | `/*! ... */`   |
| outer | `#[ATTR]`  | `///`         | `/** ... */`   |
*/

// A line non-documentation comment `//`
/* A block non-documentation comment `/* ... */`

- use M-x 'compile' to get expanded version of the source code
- use M-x 'rust-compile' to actually compile the source code
- option '--pretty' requires rustc's unstable options which requires 'nightly'.
  - cargo +nightly rustc -- -Z unpretty=expanded  > play-rust_expanded.rs
  - cargo +nightly rustc -- -Z unpretty=hir       > play-rust_hir.txt
  - cargo +nightly rustc -- -Z unpretty=hir,typed > play-rust_thir.txt
  - cargo +nightly rustc -- -Z unpretty=mir       > play-rust_mir.txt
 */

/// # Module to say Hello, World!
///
/// A line documentation outer comment with _three_ slashes `///`
///
/**
A block documentation outer comment can be nested `/** ... */`

File `hello.md` is included here in `main.rs` but belongs to module `hello`.
*/
#[doc = include_str!("./hello.md")]
// Module provides "Hello" portion of the output
mod hello;

#[doc = include_str!("./world/mod.md")]
// Module provides "World" portion of the output
mod world;

/// `play-rust`'s `main` function to play with rust.
///
// Including documentation from a text file requires 2021 edition (Version 1.54.0)
#[doc = include_str!("./main.md")]
fn main() -> () {
    // Hello-World's part "World" is created by module `hello` and `world`.
    println!(
        "{}, {}! From Modules 'hello' and 'world'",
        hello::hello(),
        world::world()
    );
    // Hello-World's part "World" is created by crate-library `world`
    println!(
        "Hello, {}! From Library 'world'", 
        ::world::world()
    );

    let a_char: char = '❤'; // UTF-8
    std::assert!(
        4 == std::mem::size_of::<char>(),
        "Type 'char' has no length of 4 on this platform"
    );
    println!("Hello, World! comes with {} from Tilo", a_char);
}
