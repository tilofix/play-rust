//! Crate to say World!

///```rust
/// assert_eq!("World (lib)", world::world());
///```
pub fn world() -> &'static str {
    let a_str_ref: &str = "World (lib)";

    /*
    A `str` (string slice) is the most primitive string type.
    It is usually seen in its borrowed form, `&str`.
    It is also the type of string literals, `&'static str`.
    A `&str` is made up of two components: 
    - a pointer to some bytes,
    - and a length.
    */
    std::assert_eq!(
        std::any::TypeId::of::<*const u8>(),
        std::any::Any::type_id(&a_str_ref.as_ptr())
    );
    std::assert_eq!(11, a_str_ref.len());
    
    a_str_ref
}

#[test]
fn returns_world() {
    assert_eq!("World (lib)", world());
}
