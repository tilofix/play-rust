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
    // println!("{:?}", String::from("World (mod)").into_bytes());
    // We need to declare the reference with static region
    // as we cannot return a local variable (from the stack) of function
    let a_u8_slice_ref: &'static [u8; 11] = &[87, 111, 114, 108, 100, 32, 40, 109, 111, 100, 41];

    ::std::assert_eq!(
        ::std::mem::size_of::<[u8; 11]>(),
        ::std::mem::size_of_val(a_u8_slice_ref)
    );

    // Need to unwrap refernce to str from Result<&str, std::str::Utf8Error>
    // which returns a str slice instead of 'Ok("World (mod)")'
    ::std::result::Result::unwrap(::std::str::from_utf8(a_u8_slice_ref))
}
