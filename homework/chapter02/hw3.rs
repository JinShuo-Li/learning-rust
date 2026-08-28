// ============================================================
// Homework -- Chapter 02, Problem 3: Result and expect
// ============================================================
//
// From the notes: `read_line` returns a `Result`, which is either
// `Ok` or `Err`. `.expect("msg")` handles it: on `Err` the program
// PANICS with "msg"; on `Ok` it continues. If you ignore a
// `Result`, the code still compiles -- you just get a warning:
//
//     warning: unused `Result` that must be used
//     = note: use `let _ = ...` to ignore the resulting value
//
// TASK:
//   1. Compile and run the program as-is:
//
//          rustc --edition 2021 hw3.rs && ./hw3
//
//      You'll see the warning the notes showed. In which
//      situation would this program misbehave? (Hint: what does
//      the notes say warnings are good for?)
//
//   2. Now handle the `Result` properly: assign it to a binding
//      and print how many BYTES were read. `read_line` returns
//      `Ok(bytes_read)` on success -- display it like:
//
//          You typed 4 bytes   (for input "hi\n")
//
//      Keep `.expect(...)` for the failure case.
//
//   3. Finally, answer in a comment: when `expect` panics, does
//      the rest of `main` still run? Verify by adding a
//      `println!` after the read and forcing an error with
//      `echo | ./hw3` (this closes stdin, so the read fails).
// ============================================================

use std::io;

fn main() {
    println!("Type something:");

    let mut input = String::new();

    // Step 1: compile me first and read the warning.
    let bytes = io::stdin().read_line(&mut input).expect("Failed to read line");

    // TODO Step 2: bind the Result, expect on Err, print bytes read
    // and echo the input back, e.g. `You typed 4 bytes: "hi"`.
    println!("You typed {} bytes: {:?}", bytes, input.trim());

    // TODO Step 3: a println! that proves whether code after a
    // panic runs -- plus your answer in comments.
    println!("This line proves whether code after panic runs.");
}
