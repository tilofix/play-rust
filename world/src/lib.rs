//! Crate to say World!

///```rust
/// assert_eq!("World (lib)", world::world());
///```
pub fn world() -> &'static str {
    let a_str_ref: &str = "World (lib)";
    assert_eq!(
        std::mem::size_of::<[u8; 11]>(),
        std::mem::size_of_val(a_str_ref)
    );
    a_str_ref
}

#[test]
fn returns_world() {
    assert_eq!("World (lib)", world());
}
