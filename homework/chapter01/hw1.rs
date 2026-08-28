// ============================================================
// Homework -- Chapter 01, Problem 1: First Output
// ============================================================
//
// From the notes, a minimal Rust program needs:
//   1. a `main` function -- it is always the first code that
//      runs in every executable Rust program,
//   2. `println!` -- note the `!`: it calls a MACRO, a way to
//      write code that generates code,
//   3. a string argument, like "Hello, world!",
//   4. a semicolon `;` to end the expression.
//
// TASK:
//   Fill in `fn main()` so the program prints EXACTLY these
//   three lines (nothing more, nothing less):
//
//       Rust is compiled.
//       Rust is fast.
//       Rust is safe.
//
//   Rules: use `println!` three times, once per line, and end
//   every statement with a semicolon.
//
// Compile & run (no cargo needed for this one):
//
//     rustc --edition 2021 hw1.rs && ./hw1
// ============================================================

fn main() {
    println!("Rust is compiled.");
    println!("Rust is fast.");
    println!("Rust is safe.");
}
