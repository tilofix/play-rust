//! Crate to say World!

///```rust
/// assert_eq!("World", world::world());
///```
pub fn world() -> &'static str {
    "World"
}

#[test]
fn returns_world() {
    assert_eq!("World", world());
}
