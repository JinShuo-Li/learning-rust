# Homework -- Chapter 01, Problem 2: The Cargo Drill

The notes introduced four cargo commands: `cargo new`, `cargo build`,
`cargo run`, and `cargo check`. This problem makes you use all of them
and see the differences with your own eyes.

> This repo is a cargo **workspace**, so after `cargo new` you must
> also add the new folder to `members` in the root `Cargo.toml`.
> The repo's own checker will find it if you forget:
>
>     python3 doctor.py --workspace

## Tasks

1. Inside `homework/chapter01/`, create a new project (no nested git,
   as the notes recommend for this repo):

       cd homework/chapter01
       cargo new hw_hello --vcs none

   Then add `"homework/chapter01/hw_hello"` to `members` in the root
   `Cargo.toml` and run `python3 doctor.py --workspace` to confirm the
   repo is healthy again.

2. Edit `hw_hello/src/main.rs` so it prints `Hello, cargo!`.

3. Run `cargo check`. Question A: did it produce an executable?
   Look for one under `target/` to verify your answer.

4. Run `cargo check` twice, then `cargo build` twice, and compare how
   long each second run takes. Question B: the notes say `cargo check`
   "quickly checks your code ... but doesn't produce an executable" --
   based on what you observed, when would you prefer `check` over
   `build` while coding?

5. Break the code on purpose: delete one semicolon in `main.rs`, then
   run `cargo check` and `cargo build`. Question C: which of the two
   reported the error faster, and did either produce an executable?
   Restore the semicolon afterwards.

6. Run the program with `cargo run`. Then, from inside `hw_hello/`,
   also try the raw-compiler way from the notes:

       rustc src/main.rs

   Question D: list the files each method produced and where they
   landed (`./main` next to the source vs. somewhere under
   `target/debug/`).

## How to submit your answers

Write your answers to questions A-D as `//` comments at the top of
`hw_hello/src/main.rs`, below a `// ANSWERS:` marker.
