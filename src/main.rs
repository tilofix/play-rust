// option '--pretty' requires rustc's unstable options which requires 'nightly'.
// -*- compile-command: "cargo +nightly rustc -- -Z unstable-options --pretty expanded" -*-

fn main() -> () {
    let x: i32 = 5;
    let _v: Vec<u64> = vec![1,2,3];
    let s: &str = "Tilo";
    println!("Hello, world! x is {}; s is {}", x, s);
}
