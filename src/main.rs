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

//!
//! For learning we make no use of
//! [Use declaration](https://doc.rust-lang.org/stable/reference/items/use-declarations.html)
//! 
//! Above that for learning we don't bring
//! [The Rust Perlude](https://doc.rust-lang.org/reference/names/preludes.html)
//! into [scope](https://doc.rust-lang.org/reference/names.html?highlight=scope#names)
/*!
```rust
#![no_implicit_prelude]
```
 */
#![no_implicit_prelude]

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
/// Including documentation from a text file requires 2021 edition (Version 1.54.0)
#[doc = include_str!("./main.md")]
fn main() -> () {
    // Hello-World's part "World" is created by module `hello` and `world`.
    ::std::println!(
        "{}, {}! From Modules 'hello' and 'world'",
        hello::hello(),
        world::world()
    );
    // Hello-World's part "World" is created by crate-library `world`
    ::std::println!(
        "Hello, {}! From Library 'world'", 
        ::world::world()
    );

    let a_char: ::std::primitive::char = '❤'; // UTF-8
    ::std::println!("Hello, World! comes with {} from Tilo", a_char);

    is_char_len(4);
}

/// Checks the length of type 'char'
///
/// ## Rust error with my attempt to do `no_implicit_prelude`
///
/// Usage of macro [`std::assert!`](https://doc.rust-lang.org/std/macro.assert.html) 
/// has let me run in an Rust error with my attempt to do `no_implicit_prelude`
/// "error: cannot find macro panic in this scope [#78333](https://github.com/rust-lang/rust/issues/78333)"
///
/// Macro `std::assert!` asserts that a boolean expression is true at runtime.
/// This will invoke the [`panic!` macro](https://doc.rust-lang.org/core/macro.panic.html)
/// if the provided expression cannot be evaluated to true at runtime.
///
/**

Here is compiler's output with verbose information.
```text

cargo rustc -v -- -Z macro-backtrace
       Fresh world v0.1.0 (/home/tilo/Projects/play-rust/world)
   Compiling play-rust v0.2.0 (/home/tilo/Projects/play-rust)
     Running `rustc --crate-name play_rust --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -Z macro-backtrace -C metadata=46634ed49d8ee5d0 -C extra-filename=-46634ed49d8ee5d0 --out-dir /home/tilo/Projects/play-rust/target/debug/deps -C incremental=/home/tilo/Projects/play-rust/target/debug/incremental -L dependency=/home/tilo/Projects/play-rust/target/debug/deps --extern world=/home/tilo/Projects/play-rust/target/debug/deps/libworld-1074c99c440ff4d9.rlib`
error: cannot find macro `panic` in this scope
    --> src/main.rs:153:5
     |
153  | /     ::std::assert!(
154  | |         len == ::std::mem::size_of::<::std::primitive::char>(),
155  | |         "Type 'char' has no length of 4 on this platform"
156  | |     );
     | |     ^ in this macro invocation
     | |_____|
     | 
     |
    ::: /home/tilo/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/macros/mod.rs:1329:5
     |
1329 | /     macro_rules! assert {
1330 | |         ($cond:expr $(,)?) => {{ /* compiler built-in */ }};
1331 | |         ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};
1332 | |     }
     | |_____- in this expansion of `::std::assert!`
     |
     = note: consider importing one of these items:
             std::panic
             core::panic

error: could not compile `play-rust` due to previous error

```

 *Issue* "error: cannot find macro panic in this scope"
[#78333](https://github.com/rust-lang/rust/issues/78333)
is fixed by *Pull-Request* "Qualify panic! as core::panic! in non-built-in core macros"
[#78343](https://github.com/rust-lang/rust/pull/78343)
which [bors](https://github.com/bors)
merged with *Commit* "Auto merge of #78343 - camelid:macros-qualify-panic, r=m-ou-se"
[f32a0cc](https://github.com/rust-lang/rust/commit/f32a0cce2fd5aaf5f361192af59cf1f2afa5f0ac)
into rust-lang:master on Nov 24, 2020.
and finally [rustbot](https://github.com/rustbot)
added this to the [1.50.0](https://github.com/rust-lang/rust/milestone/77)
milestone on Nov 24, 2020

Following `rustc` version has been used.
```text

cargo rustc -- -V -v -Z macro-backtrace
   Compiling play-rust v0.2.0 (/home/tilo/Projects/play-rust)
rustc 1.59.0-nightly (efec54529 2021-12-04)
binary: rustc
commit-hash: efec545293b9263be9edfb283a7aa66350b3acbf
commit-date: 2021-12-04
host: x86_64-unknown-linux-gnu
release: 1.59.0-nightly
LLVM version: 13.0.0

```

Here is what I have installed. Couldn't get version information of rustup's installed components.
```text

[tilo@holm src]$ rustup check
stable-x86_64-unknown-linux-gnu - Update available : 1.57.0 (f1edd0429 2021-11-29) -> 1.76.0 (07dca489a 2024-02-04)
nightly-x86_64-unknown-linux-gnu - Update available : 1.59.0-nightly (efec54529 2021-12-04) -> 1.78.0-nightly (2d24fe591 2024-03-09)
rustup - Up to date : 1.26.0

```

Oh, this commit has first release tag [1.50.0](https://github.com/rust-lang/rust/releases/tag/1.50.0).
So it is already in my stable and nightly installation.

Indeed output of "Nightly builds, run with -Z macro-backtrace for more info"
says `compiler built-in`, means `panic` is not called by library but compiler.

Source code analysis:
- Sieht so aus als ob der Makro hier registriert wird:
  [`assert: assert::expand_assert`](https://github.com/rust-lang/rust/blob/af69f4c48c1e1cfbb9ba43b376edcfbdd8cfbca4/compiler/rustc_builtin_macros/src/lib.rs#L78).
- Ha, hier haben wir es:
  [`panich_path`](https://github.com/rust-lang/rust/blob/af69f4c48c1e1cfbb9ba43b376edcfbdd8cfbca4/compiler/rustc_builtin_macros/src/assert.rs#L35)
  wird seit Edition 2021 anders behandelt;
  - On edition 2021, we always call `$crate::panic::panic_2021!()`.
  - Before edition 2021, we call `panic!()` unqualified,
    such that it calls either `std::panic!()` or `core::panic!()`.

Also muss ich mit `--edition=2018` die Use-Deklaration vor der Vewendung von `::std::assert!`
einsetzen:
```rust
use ::std::panic;
```

On my Windows host I've got rust versions
`1.62.1 (e092d0b6b 2022-07-16)` and `1.64.0-nightly (f9cba6374 2022-07-31)`.
Whereas on my Gnu/Linux host I have versions
`1.57.0 (f1edd0429 2021-11-29)` and `1.59.0-nightly (efec54529 2021-12-04)`.
So I gave it try. New Rust version isn't sufficient,
I needed also to move the project from edition `2018` to `2021`.

```text
PS C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust>
PS C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust> rustup check
warning: Signature verification failed for 'https://static.rust-lang.org/dist/channel-rust-stable.toml'
stable-x86_64-pc-windows-msvc - Update available : 1.62.1 (e092d0b6b 2022-07-16) -> 1.76.0 (07dca489a 2024-02-04)
warning: Signature verification failed for 'https://static.rust-lang.org/dist/channel-rust-nightly.toml'
nightly-x86_64-pc-windows-msvc - Update available : 1.64.0-nightly (f9cba6374 2022-07-31) -> 1.78.0-nightly (4a0cc881d 2024-03-11)
rustup - Update available : 1.25.1 -> 1.27.0
PS C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust>

PS C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust> cargo +nightly -v clean
    Removing C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target
PS C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust>

PS C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust>
PS C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust> cargo +nightly -v build
p\c_twirkner\Projects\play-rust\target\debug\deps -C incremental=C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target\debug\incremental -L dependency=C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target\debug\deps --extern world=C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target\debug\deps\libworld-5e506bbab6a5ce27.rlib`
error: cannot find macro `panic` in this scope
   --> src\main.rs:216:5
    |
216 | /     ::std::assert!(
217 | |         len == ::std::mem::size_of::<::std::primitive::char>(),
218 | |         "Type 'char' has no length of 4 on this platform"
219 | |     );
    | |_____^
    |
    = note: consider importing one of these items:
            std::panic
            core::panic
    = note: this error originates in the macro `::std::assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error: could not compile `play-rust` due to previous error

Caused by:
  process didn't exit successfully: `rustc --crate-name play_rust --edition=2018 src\main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=94440001cf4e6dbf --out-dir C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target\debug\deps -C incremental=C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target\debug\incremental -L dependency=C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target\debug\deps --extern world=C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target\debug\deps\libworld-5e506bbab6a5ce27.rlib` (exit code: 1)
PS C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust>
PS C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust>
PS C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust> cargo +nightly -v clean
    Removing C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target
PS C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust>

PS C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust>
PS C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust> cargo +nightly -v build
   Compiling world v0.1.0 (C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\world)
     Running `rustc --crate-name world --edition=2018 world\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=5e506bbab6a5ce27 -C extra-filename=-5e506bbab6a5ce27 --out-dir C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target\debug\deps -C incremental=C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target\debug\incremental -L dependency=C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target\debug\deps`
   Compiling play-rust v0.2.0 (C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust)
     Running `rustc --crate-name play_rust --edition=2021 src\main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=94440001cf4e6dbf --out-dir C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target\debug\deps -C incremental=C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target\debug\incremental -L dependency=C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target\debug\deps --extern world=C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust\target\debug\deps\libworld-5e506bbab6a5ce27.rlib`
    Finished dev [unoptimized + debuginfo] target(s) in 3.38s
PS C:\Users\Public\Downloads\Backup\c_twirkner\Projects\play-rust> 
```

On my GNU/Linux host the same version compiled with Rust edition 2021.

There had been more issues, here is another one closed to "mine"
"`::core::assert!` in `#![no_implicit_prelude]` fails to compile because
it calls `panic!` instead of `::core::panic!` [#84357](https://github.com/rust-lang/rust/issues/84357)

So my Rust source code investigation
(master ([#93179](https://github.com/rust-lang/rust/pull/93179)); 1.76.0 ... 1.60.0)
had been wrong with respect to issue `error: cannot find macro `panic` in this scope`.
It is not only solved in version `1.60.0` but already in `1.59.0` as well as `1.57.0`.

## Rust Editions

Rust 2015
- It commenced with the release of 1.0, and is the "default edition".
- The edition system was conceived in late 2017, but Rust 1.0 was released in May of 2015.
- As such, 2015 is the edition that you get when you don't specify any particular edition,
  for backwards compatibility reasons.

Rust 2018
- RFC : [#2052](https://rust-lang.github.io/rfcs/2052-epochs.html), which also proposed the Edition system
- Release version : [1.31.0](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)

Rust 2021
- RFC : [#3085](https://github.com/rust-lang/rfcs/pull/3085)
- Release version : [1.56.0](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html)

PS: zum wegmerken : MBE: more checks at definition time [#61053](https://github.com/rust-lang/rust/issues/61053)

 */
fn is_char_len(len: usize) -> () {
    //use ::std::panic;
    ::std::assert!(
        len == ::std::mem::size_of::<::std::primitive::char>(),
        "Type 'char' has no length of 4 on this platform"
    );
}
