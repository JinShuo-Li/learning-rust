// ============================================================
// Homework -- Chapter 02, Problem 2: Mutability and Shadowing
// ============================================================
//
// From the notes:
//   - variables are IMMUTABLE by default; `mut` makes them
//     changeable,
//   - `let guess: u32 = guess.trim().parse()...` re-declares a
//     variable with the SAME name and a DIFFERENT type. This is
//     called SHADOWING.
//
// TASK: this program compiles today, but it shouldn't! Your job:
//
//   1. Run it as-is:
//
//          rustc --edition 2021 hw2.rs && ./hw2
//
//      and note what the two counters print.
//
//   2. Part A -- remove `mut` from the `counter` line. Recompile.
//      Read the compiler error carefully: what exactly does it
//      complain about? Write the answer as a comment.
//
//   3. Part B -- the two `value` lines demonstrate shadowing.
//      Below them, add a line that multiplies `value` by 10 and
//      prints it. Then answer in a comment: does the compiler
//      let you multiply a `String` by 10? Which `value` is alive
//      at that point -- the `&str` one or the `String` one?
//
//   4. Part C -- explain in a comment: with shadowing, why is it
//      OK that `value` was never declared `mut`, even though we
//      "changed" it from "42" to 42-ish below?
// ============================================================

fn main() {
    let mut counter = 0;
    // Answer: the compiler complained: "cannot assign twice to immutable variable `counter"

    counter = counter + 1;
    println!("counter = {counter}");

    let value = "42";              // a string literal
    let value = value.len();       // shadowing: now a number
    println!("value = {value}");

    // TODO Part B: multiply value by 10, print it, answer the
    // questions in comments.

    // Answer: cannot multiply `&str` by `{integer}`
    // Answer: the value that is assigned by `value.len()` alive here

    // TODO Part C: your explanation comment goes here.
    // Answer: Because the two values before and after don’t point to the same memory.
    // Even though they have the same name, they actually don’t belong to the same variable due to the difference in data types.
    // The latter just shadows the former, rather than changing its value.
}
