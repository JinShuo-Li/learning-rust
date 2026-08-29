// ============================================================
// Homework -- Chapter 02, Problem 7: Ordering and match
// ============================================================
//
// From the notes: `guess.cmp(&secret)` returns an `Ordering`
// enum with exactly three variants -- `Less`, `Greater`,
// `Equal` -- and a `match` handles every possible outcome.
//
// TASK: temperature report. Complete the program so that:
//   1. It reads an integer temperature (in Celsius) from stdin,
//      parsing exactly like the notes do (trim + parse + expect).
//   2. It compares `temp` against the THRESHOLD constant below
//      with `.cmp()` and a `match`, printing:
//
//          Ordering::Less    -> "Below threshold"
//          Ordering::Greater -> "Above threshold"
//          Ordering::Equal   -> "Exactly at threshold"
//
//   3. PROVE exhaustiveness: after your match works, temporarily
//      delete the `Ordering::Equal` arm and recompile. Record
//      the compiler error in a comment (one or two lines of it),
//      then restore the arm. This is the compiler guarantee the
//      notes advertise -- match must cover every variant.
//
// Expected run:
//
//     $ rustc --edition 2021 hw7.rs && ./hw7
//     Enter temperature: 25
//     Comparing 25 with threshold 20: Above threshold
//
//     (with input 20 -> "Exactly at threshold",
//      with input 15 -> "Comparing 15 with threshold 20: Below threshold")
// ============================================================

use std::cmp::Ordering;
use std::io;

const THRESHOLD: i32 = 20;

fn main() {
    println!("Enter temperature: ");

    // TODO: read temp, then match temp.cmp(&THRESHOLD) { ... }
    let mut temper = String::new();

    io::stdin()
        .read_line(&mut temper)
        .expect("Failed to read line");

    let temper: i32 = temper
        .trim().parse()
        .expect("Failed to convert the data class");

    match temper.cmp(&THRESHOLD) {
        Ordering::Less => println!("Comparing {temper} with threshold {THRESHOLD}: Below threshold"),
        Ordering::Equal => println!("Exactly at threshold"),
        Ordering::Greater => println!("Comparing {temper} with threshold {THRESHOLD}: Above threshold"),
    }
}
