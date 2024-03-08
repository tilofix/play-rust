#![feature(prelude_import)]
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
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;

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

// Module provides "Hello" portion of the output
/// # Module to say Hello, World!
///
/// A line documentation outer comment with _three_ slashes `///`
///
/**
A block documentation outer comment can be nested `/** ... */`

File `hello.md` is included here in `main.rs` but belongs to module `hello`.
*/
#[doc =
  "## Module hello to learn including modules\n\nThis text is in file `hello.md`\nand is included in source module `main.rs`\nto document module `hello`.\n\n"]
mod hello {

    // Module provides "World" portion of the output

    // Including documentation from a text file requires 2021 edition (Version 1.54.0)
    // Hello-World's part "World" is created by module `hello` and `world`.
    // Hello-World's part "World" is created by crate-library `world`

    // UTF-8
    //! ## Module to return Hello
    //!
    #![doc =
       "Here is an inner doc-attribute comment 
to document its parent, the file and thus module `hello.rs`."]
    //!
    /*!
A module is a container for zero or more __items__.

This module defined in file `hello.rs`
could have also been defined in `hello/mod.rs`.
As file `main.rs` "includes" this with statement `mod hello;`
the definition in file `hello.rs` becomes a module.

Alternatively module `hello` could have also been defined
with help of
```no_run
mod hello {
  pub fn hello() {
  }
}
```
*/
    /// Function that print line 'Hello, World!' from module `hello`
    ///
    /// This is a line documentation outer comment.
    #[doc = "This is an outer doc-attribute comment
to document its ."]
    /// # Examples
    ///
    /// ```rust
    /// hello();
    /// ```
    pub fn hello() -> String {
        let a_string: std::string::String = String::from("Hello");
        {
            match (&std::any::TypeId::of::<*const u8>(),
                   &std::any::Any::type_id(&a_string.as_ptr())) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(kind, &*left_val,
                                                         &*right_val,
                                                         ::core::option::Option::None);
                    }
                }
            }
        };
        {
            match (&5, &a_string.len()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(kind, &*left_val,
                                                         &*right_val,
                                                         ::core::option::Option::None);
                    }
                }
            }
        };
        {
            match (&5, &a_string.capacity()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(kind, &*left_val,
                                                         &*right_val,
                                                         ::core::option::Option::None);
                    }
                }
            }
        };
        a_string
    }
}
#[doc =
  "## Module world to learn including modules\n\nThis text is in file `world.md`\nand is included in source module `main.rs`\nto document module `world`.\n\n"]
mod world {
    //! ## Module to return World
    //!
    /// Function world() returns a static reference to a str slice
    ///
    /// The returned reference has to have a [`static` lifetime](https://doc.rust-lang.org/std/keyword.static.html)
    /// makeing the reference valid for the entire duration of the program
    /// because the referenced value is borrowed from the function.
    ///
    ///```rust
    /// assert_eq!("World (mod)", world());
    ///```
    pub fn world() -> &'static str {
        let a_u8_slice_ref: &'static [u8; 11] =
            &[87, 111, 114, 108, 100, 32, 40, 109, 111, 100, 41];
        {
            match (&std::mem::size_of::<[u8; 11]>(),
                   &std::mem::size_of_val(a_u8_slice_ref)) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(kind, &*left_val,
                                                         &*right_val,
                                                         ::core::option::Option::None);
                    }
                }
            }
        };
        std::result::Result::unwrap(std::str::from_utf8(a_u8_slice_ref))
    }
}
/// `play-rust`'s `main` function to play with rust.
///
#[doc =
  "## Some explanation of rust-code in documentation.\n\nWhat do you think, is rust a simple language?\n\nI don\'t think so, please press the \"Run\" button,\nselect `TOOLS` from the right hand drop down menu,\nand execute `Expand marcos`.\n```rust\nprintln!(\"{}\", \"Hello, World!\");\n```\n\nNow you see the truth,\nwhat rust is really about.\nPrint line \"Hello World\" is a complicated think in rust, isn\'t it?\n```rust\n# #![feature(fmt_internals)]\n# #![feature(print_internals)]\n#![feature(prelude_import)]\n#[prelude_import]\nuse std::prelude::rust_2018::*;\n#[macro_use]\nextern crate std;\nfn main() {\n    {\n        ::std::io::_print(::core::fmt::Arguments::new_v1(\n            &[\"\", \"\\n\"],\n            &match (&\"Hello, World!\",) {\n                _args => [::core::fmt::ArgumentV1::new(\n                    _args.0,\n                    ::core::fmt::Display::fmt,\n                )],\n            },\n        ));\n    };\n}\n```\n"]
fn main() -> () {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["", ", ",
                                                           "! From Modules \'hello\' and \'world\'\n"],
                                                         &match (&hello::hello(),
                                                                 &world::world())
                                                              {
                                                              _args =>
                                                              [::core::fmt::ArgumentV1::new(_args.0,
                                                                                            ::core::fmt::Display::fmt),
                                                               ::core::fmt::ArgumentV1::new(_args.1,
                                                                                            ::core::fmt::Display::fmt)],
                                                          }));
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["Hello, ",
                                                           "! From Library \'world\'\n"],
                                                         &match (&::world::world(),)
                                                              {
                                                              _args =>
                                                              [::core::fmt::ArgumentV1::new(_args.0,
                                                                                            ::core::fmt::Display::fmt)],
                                                          }));
    };
    let a_char: char = '❤';
    if !(4 == std::mem::size_of::<char>()) {
        {
            ::std::rt::begin_panic("Type 'char' has no length of 4 on this platform")
        }
    };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["Hello, World! comes with ",
                                                           " from Tilo\n"],
                                                         &match (&a_char,) {
                                                              _args =>
                                                              [::core::fmt::ArgumentV1::new(_args.0,
                                                                                            ::core::fmt::Display::fmt)],
                                                          }));
    };
}
