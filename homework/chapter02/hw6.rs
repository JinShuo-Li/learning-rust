// ============================================================
// Homework -- Chapter 02, Problem 6: trim, parse, and Types
// ============================================================
//
// From the notes: before parsing, we call `trim()` to remove
// whitespace (including the newline `read_line` leaves behind),
// then `parse()` to convert the String into a number. The target
// type must be known -- either from the type annotation on the
// binding, like `let guess: u32 = ...`, or from how the value is
// used.
//
// TASK: make this program compile and run correctly. Expected
// interaction:
//
//     $ rustc --edition 2021 hw6.rs && ./hw6
//     Enter a number:  42
//     Doubled: 84
//     Half:   21
//     As f64: 84.0
//
// (Note the stray spaces before 42 -- that's on purpose, to make
// trim() earn its keep.)
//
// Steps:
//   1. Bring `std::io` into scope.
//   2. Read a line into a mutable String.
//   3. Parse it into a number type that works for ALL the
//      operations below. Annotate the binding with its type.
//      Hint: doubling and halving keep the "half" exact only for
//      even inputs with integer types; "Half: 21" is exact for
//      42, so an integer type is fine -- pick the one the notes
//      used for guesses.
//   4. Print the three lines using `{...}` placeholders.
//
// Bonus: try entering "4o2" (a typo). What happens, and why is
// the message what it is? Answer in a comment.
// ============================================================

fn main() {
    println!("Enter a number:");

    // TODO: read, trim, parse (with type annotation), print.
}
