//! ## Module to print Hello, World!
//!
#![doc = "Here is an inner doc-attribute comment 
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

pub fn hello() {
    println!("Hello, World! From Module 'hello'")
}
