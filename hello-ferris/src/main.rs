// -*- compile-command: "cargo +nightly rustc -- -Z unpretty=hir" -*-
// Small application from Rust | Getting started
// https://www.rust-lang.org/learn/get-started

// no use statement for learning
// use std::io::{stdout, Write, BufWriter};
// no Rust prelude for learning
#![no_implicit_prelude]

fn main() {
    // stdout : a handle to a shared global buffer whose access is synchronized via a mutex
    let stdout: ::std::io::Stdout = ::std::io::stdout();
    // lock(&self) : locks this handle to the standard output stream,
    //   returning a guard implementing the Write trait for writing data.
    // The lock is released when the returned lock goes out of scope.
    let mut writer: ::std::io::BufWriter<::std::io::StdoutLock<'_>> =
        ::std::io::BufWriter::new(stdout.lock());

    let ref_array_u8_message: &[u8] = "Hello, fellow Rustaceans!".as_bytes();

    ::ferris_says::say(
        ref_array_u8_message,
        ref_array_u8_message.len(),
        &mut writer,
    )
    .unwrap();
    //writer.flush().unwrap();
    //        ^^^^^ method not found in `BufWriter<StdoutLock<'_>>`
    //
    // help: items from traits can only be used if the trait is in scope
    // help: the following trait is implemented but not in scope; perhaps add a `use` for it:
    // use std::io::Write;
    ::std::io::Write::flush(&mut writer).unwrap();

    #[derive(Debug)]
    struct Message {
        // Box<str> to avoid lifetime required in case of borrowed '&str'
        greeting: ::std::boxed::Box<str>,
        greet_to: ::std::boxed::Box<str>,
    }

    let struct_message = Message {
        greeting: ::std::borrow::ToOwned::to_owned("Hello").into_boxed_str(),
        greet_to: ::std::borrow::ToOwned::to_owned("Fellow Rustaceans").into_boxed_str(),
    };

    ::std::println!("{:?}", struct_message);

    let array_boxed_str_message = [
        struct_message.greeting,
        ::std::borrow::ToOwned::to_owned(", ").into_boxed_str(),
        struct_message.greet_to,
    ];
    ::ferris_says::say(
        array_boxed_str_message.concat().as_str().as_bytes(),
        array_boxed_str_message.len(),
        &mut writer,
    )
    .unwrap();
}
