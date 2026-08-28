// ============================================================
// Homework -- Chapter 01, Problem 3: Fix Three Bugs
// ============================================================
//
// The code below violates THREE different rules from the notes,
// so it does NOT compile. That is the point.
//
// TASK:
//   1. Try to compile it first and actually read the errors:
//
//          rustc --edition 2021 hw3.rs
//
//      You should see THREE distinct errors.
//
//   2. Fix all three bugs. Each bug breaks one of these rules
//      from the notes:
//        - `println!` calls a macro -- the `!` is not optional,
//        - we end expressions with a semicolon `;`,
//        - the `main` function is special: it is always the
//          first code that runs, so the entry point must be
//          named exactly `main` (lowercase).
//
//   3. Above each line you fix, add a short comment naming the
//      rule it broke.
//
// Expected output once fixed:
//
//     I am learning Rust.
//     Chapter 1 is done!
//     On to Chapter 2!
// ============================================================

fn Main() {
    println("I am learning Rust.");

    println!("Chapter 1 is done!")

    println!("On to Chapter 2!");
}
