// ============================================================
// Homework -- Chapter 02, Problem 8: loop, break, continue
// ============================================================
//
// From the notes: `loop` runs forever until a `break`; `continue`
// skips the rest of the current iteration. The guessing game
// uses `continue` to re-prompt after invalid input, and `break`
// to exit when the user wins.
//
// TASK: write a mini REPL. The program repeatedly:
//   1. prints "> " as a prompt and reads a line,
//   2. if the line (trimmed) is "quit", breaks the loop and
//      prints "Bye!",
//   3. if the line (trimmed) is EMPTY, `continue`s without
//      printing anything (just re-prompts),
//   4. otherwise prints "You said: <the trimmed line>".
//
// Expected interaction:
//
//     $ rustc --edition 2021 hw8.rs && ./hw8
//     > hello
//     You said: hello
//     >
//     >   spaced
//     You said: spaced
//     > quit
//     Bye!
//
// NOTE on the prompt: `println!` adds a newline; for a real "> "
// prompt that stays on the same line you'd want `print!` +
// `io::stdout().flush()`. You haven't met `flush` in the notes
// yet, so it's fine to just `println!` the prompt -- the spirit
// of the exercise is the loop, not the terminal cosmetics.
// ============================================================

use std::io;

fn main() {
    // TODO: loop { read, trim, match on "quit" / empty / else }
    loop {
        println!("> ");

        let mut prompt = String::new();
        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line");

        if prompt.trim() == "quit" {
            println!("Bye!");
            break;
        } else if prompt.trim().is_empty() {
            continue;
        } else {
            println!("You said: {}", prompt.trim());
        }
    }
}
