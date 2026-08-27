# Rust Programming Language

> Ref: *The Rust Programming Language* by Steve Klabnik, Carol Nichols, and Chris Krycho.

> All the code are compiled in WSL2, Ubuntu 24.04

## Repo Doctor

[`doctor.py`](doctor.py) is a small Python helper that keeps this repo healthy. It checks for:

1. **Stray compiled artifacts** -- e.g. executables produced by running `rustc main.rs` directly inside `src/`, which `git` does not ignore. It lists them all and asks once whether to delete them.
2. **Workspace mismatches** -- warns if a directory has a `Cargo.toml` but is missing from the root workspace `members` (or vice versa). Warning only.
3. **Nested git repositories** -- e.g. a `.git` left inside a chapter folder by `cargo new` without `--vcs none`, which would hide the code from the outer repo. Lists them all and asks once whether to delete them.

Usage (Python 3.11+, no third-party dependencies):

```
python3 doctor.py                # run all three checks in order
python3 doctor.py --binary       # only check for stray compiled artifacts
python3 doctor.py --workspace    # only check workspace member mismatches
python3 doctor.py --git          # only check for nested .git directories
python3 doctor.py --check-only   # list findings only, never prompt or delete
```

## Chapter 0: Install `Rust`

Run the following command:

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

We also need a *linker*, which is a program that Rust uses to join its compiled outputs into one file. In Linux, users should gernerally install GCC or Clang.

### Updating and Uninstalling

To update:

```
rustup update
```

To uninstall:

```
rustup self uninstall
```

## Chapter 1: Start up

### Initialization: Cargo

Now we can initialize the first rust project:

```
cargo new ch01_startup
```

If you don't want to initialize it with `git`, you can run:

```
cargo new ch01_startup --vcs none
```

Then you can see a new directory appear in your working place. This is the new rust project you just made.

In the future, to compile, run the following command:

```
cd ch01_startup
cargo build
```

To compile & run the project, use:

```
cd ch01_startup
cargo run
```

`Cargo` also provides a command called `cargo check` that can *quickly* checks your code to make sure it compiles but doesn't produce an executable.

There are also other useful `cargo` command, we *may* introduce them in later chapters.

### Hello, World!

In the rust project we just made, we can see a rust source file named `main.rs` in `src/`, which contains:

```rust
fn main() {
    println!("Hello, world!");
}
```

Let's review it in detail:

```rust
fn main() {
    // your code
}
```

These lines define a function named main. The main function is special, it is always the first code that runs in every executable Rust program. If there were parameters, they would go inside the `()`. And the function body is wrapped in curly brackets `{}`.

```rust
println!("Hello, World!");
```

This line prints text to the screen. Here are three more things we need to know:

1. `println!` calls a **Macro**, it is a way to write code that generates code to extend Rust syntax.
2. `"Hello, World!"` is a `string`, we pass this string as an argument to `println!`, and it is printed to the screen.
3. We end the line with a semicolon `;`, which indicates that this expression is over and the next one is ready to begin.

We can also compile the source code of `.rs` file without `cargo`, run:

```shell
rustc main.rs
```

Then run `./main` to see the output(s).

But we still recommend you to use `cargo`.
