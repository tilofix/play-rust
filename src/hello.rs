//! ## Module to return Hello
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

pub fn hello() -> String {
    let a_string: std::string::String = String::from("Hello");

    /*
    As String is growable, and thus not a sized type
    but a struct which is made of three components:
    - a pointer to some bytes,
    - a length,
    - and a capacity.
    */
    std::assert_eq!(
        std::any::TypeId::of::<*const u8>(),
        std::any::Any::type_id(&a_string.as_ptr())
    );
    std::assert_eq!(5, a_string.len());
    std::assert_eq!(5, a_string.capacity());

    a_string
}
