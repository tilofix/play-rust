## Some explanation of rust-code in documentation.

What do you think, is rust a simple language?

I don't think so, please press the "Run" button,
select `TOOLS` from the right hand drop down menu,
and execute `Expand marcos`.
```rust
println!("{}", "Hello, World!");
```

Now you see the truth,
what rust is really about.
Print line "Hello World" is a complicated think in rust, isn't it?
```rust
# #![feature(fmt_internals)]
# #![feature(print_internals)]
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", "\n"],
            &match (&"Hello, World!",) {
                _args => [::core::fmt::ArgumentV1::new(
                    _args.0,
                    ::core::fmt::Display::fmt,
                )],
            },
        ));
    };
}
```
