# Homework -- Chapter 02, Problem 5: Add a Dependency the Right Way

The notes showed that using the `rand` crate takes three steps:
write `use rand::Rng;`, add `rand = "0.8.5"` under
`[dependencies]` in `Cargo.toml`, and let cargo fetch it. This
problem walks you through it inside this repo's workspace.

## Tasks

1. Create a new binary crate inside the homework folder and
   register it in the workspace (same drill as chapter01 hw2):

       cd homework/chapter02
       cargo new hw_crate --vcs none

   Add `"homework/chapter02/hw_crate"` to `members` in the root
   `Cargo.toml`, then confirm with `python3 doctor.py --workspace`.

2. Add to `hw_crate/Cargo.toml`:

       [dependencies]
       rand = "0.8.5"

3. In `hw_crate/src/main.rs`, write a program that:
   - brings the `Rng` trait into scope (the notes stress: without
     the `use rand::Rng;` line, `gen_range` doesn't compile),
   - generates a secret number in `1..=100`,
   - reads a guess from stdin, trims and parses it to `u32` with
     `.expect("Please type a number!")`,
   - prints whether the guess was too small / too big / correct,
     using `std::cmp::Ordering` and a `match`, exactly like the
     notes' third version of the game.

4. Run it with `cargo run` (from anywhere in the workspace,
   `cargo run -p hw_crate` also works). Play a few rounds.

5. Now break it on purpose: comment out ONLY the `use rand::Rng;`
   line and run `cargo check`. Question A: quote the compiler
   error, and explain in one sentence why bringing the *trait*
   into scope is required to call `gen_range`. Restore the line.

6. Question B: run `cargo update` and then inspect `Cargo.lock`
   (repo root). What did the update change, and why is the
   version still `0.8.x` and not `0.9.x`? (Hint: semver --
   `rand = "0.8.5"` means `>=0.8.5, <0.9.0`.)

## How to submit your answers

Write your answers to A and B as `//` comments at the top of
`hw_crate/src/main.rs`, under a `// ANSWERS:` marker.
