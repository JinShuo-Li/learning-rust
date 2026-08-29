// ============================================================
// Homework -- Chapter 02, Problem 9: Match on Result
// ============================================================
//
// From the notes: the final game replaced
// `.expect("Please type a number!")` with a `match` on the
// `Result` returned by `parse()`:
//
//     let guess: u32 = match guess.trim().parse() {
//         Ok(_num) => _num,
//         Err(_num) => continue,
//     };
//
// ...so bad input no longer crashes the program.
//
// TASK: robust input loop. The program asks for a number, and:
//   - on `Ok(n)`: prints "Got: {n}" and exits,
//   - on `Err(_)`: prints "Not a number, try again." and asks
//     again (loop!),
//   - on EOF/closed stdin: the read fails; the program panics
//     via expect -- that's acceptable here, matching the notes.
//
// Expected interaction:
//
//     $ rustc --edition 2021 hw9.rs && ./hw9
//     Enter a number: abc
//     Not a number, try again.
//     Enter a number: 7x
//     Not a number, try again.
//     Enter a number: 7
//     Got: 7
//
// Rules:
//   - NO `.expect` on the parse -- the whole point is the match,
//   - structure it like the notes: read inside a loop, parse
//     into a shadowed variable, `match` the Result, `break` or
//     `continue` accordingly.
// ============================================================

use std::io;

fn main() {
    // TODO: loop { read, match on parse() Result }
    loop {
        println!("Enter a number:");

        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        let num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number, try again.");
                continue;
            }
        };

        println!("Got: {num}");
        break;
    }
}
