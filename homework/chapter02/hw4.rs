// ============================================================
// Homework -- Chapter 02, Problem 4: Number Guessing Warm-up
// ============================================================
//
// From the notes: `rand::thread_rng().gen_range(1..=100)`
// generates a number in [1, 100]. The range syntax matters:
//   - `1..=100` is INCLUSIVE of 100,
//   - `1..100`  is EXCLUSIVE of 100 (so [1, 99]).
//
// TASK: write a program that:
//   1. Rolls a six-sided die 10 times (range 1..=6).
//   2. Prints each roll on its own line, formatted like:
//
//          Roll 1: 4
//          Roll 2: 6
//          ...
//
//   3. After the loop, prints the total of all 10 rolls.
//
// Compile & run:
//
//     rustc --edition 2021 hw4.rs && ./hw4
//
// NOTE: `rand` is an external crate. For this problem you have
// two options:
//   a) quick way -- replace the roll with a hardcoded value and
//      focus on the loop and the sum (mark it with a TODO), or
//   b) proper way -- wait for Problem 5, where you set up a real
//      cargo project with `rand = "0.8.5"` in Cargo.toml, then
//      come back and finish this one with real randomness.
//
// Either way, the loop, formatting, and total must work.
// ============================================================

fn main() {
    // TODO: roll 10 times, print each roll, then print the total.
}
