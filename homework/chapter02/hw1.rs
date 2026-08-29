// ============================================================
// Homework -- Chapter 02, Problem 1: Reading a Line
// ============================================================
//
// From the notes: to accept user input we need the `std::io`
// library. `io::stdin()` gives us a handle to standard input, and
// `read_line(&mut guess)` stores what the user types into a
// variable -- through a MUTABLE REFERENCE (`&mut`).
//
// TASK: complete the program so it behaves like this:
//
//     $ rustc --edition 2021 hw1.rs && ./hw1
//     What is your name? Ada
//     Hello, Ada!
//
// Steps:
//   1. Bring the io library into scope with a `use` statement.
//   2. Create a new empty String, and make it mutable.
//   3. Read a line from stdin into it, handling the `Result`
//      with `.expect(...)` like in the notes.
//   4. Print a greeting with `{name}` as the placeholder.
//
// Hint: `read_line` keeps the trailing newline, and `{name}`
// inside `println!` prints the string as-is. That's fine here --
// the expected output above already ends with `!` on the same
// line as the name; if your `!` lands on the next line, trim()
// will fix it (that method shows up in Problem 6).
// ============================================================

// TODO 1: `use` the io module (place it at the top of the file)
use std::io;

fn main() {
    println!("What is your name?");

    // TODO 2: let mut name = ...;
    let mut name = String::new();

    // TODO 3: io::stdin().read_line(...).expect(...);
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // TODO 4: println!(...);
    println!("Hello, {name}")
}
