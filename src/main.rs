// -*- compile-command: "cargo +nightly rustc -- -Z unstable-options --pretty expanded" -*-
// option '--pretty' requires rustc's unstable options which requires 'nightly'.
// use M-x 'compile' to get expanded version of the source code
// use M-x 'rust-compile' to actually compile the source code

mod hello;

fn main() -> () {
    hello::hello();

    let x: i32 = 5;
    let c2: char = '❤'; // UTF-8
    let s: &str = "Tilo";
    println!("Hello, world! x is {} with a {}; s is {}", x, c2, s);
    
    let v = vec![1,2,3]; // The vec! macro for convenient initialization
    let sl: &[isize] = &v; // A Vec can be mutable. Slices are read-only objects. 
    let sl_cat: &[isize] = &[sl, &[4,5,6]].concat();
    for elem in sl_cat { println!("{}", elem); }
}
