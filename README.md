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

## Chapter 2: Programming a Guessing Game

Let's take a look at the source code of `ch02_gaming/src/main.rs`:

```rust
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

This program is a simple guessing game. Let's review it in detail:

```rust
use std::io;
```

Here, the program use the `std::io` library, which is a standard library that provides you with a number of useful features, including the ability to accept user input.

```rust
let mut guess = String::new();
```

This line creates a mutable variable named `guess` and initializes it with a new, empty `String`.

The `mut` keyword indicates that the value of `guess` can be changed later in the program. The `String::new()` function creates a new instance of a string.

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

Here we use the function `io::stdin()` from the `std::io` library to get a handle to the standard input of the terminal. Then we call the method `read_line()` on that handle, passing a mutable reference to our `guess` variable. This method will read a line of input from the user and store it in the `guess` variable.

The `&` symbol indicates that we are passing a reference to the `guess` variable, rather than the variable itself. This is necessary because `read_line()` needs to modify the value of `guess`.

**References are a complex feature, and one of Rust's major advantages is how safe and easy it is to use references.**

and:

```rust
    .expect("Failed to read line");
```

Here the program handles potential failure with the result type. The `read_line()` method returns a `Result` type, which can be either `Ok` or `Err`. If the result is `Err`, the program will panic and print the message "Failed to read line". If the result is `Ok`, the program will continue executing.

If we don't use `expect()`, and compile the program, we will get a warning:

```shell
   Compiling ch02_gaming v0.1.0 (/home/lijs/tmp/rustlearning/ch02_gaming)
warning: unused `Result` that must be used
  --> ch02_gaming/src/main.rs:9:5
   |
 9 | /     io::stdin()
10 | |         .read_line(&mut guess);
   | |______________________________^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
   |
 9 |     let _ = io::stdin()
   |     +++++++

warning: `ch02_gaming` (bin "ch02_gaming") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
```

In Rust, fun fact: Compilable warnings are not errors, and they do not prevent the program from running. As Rust designed to be strict and safe, it will warn you about potential problems in your code, even if they do not prevent the program from running. This is a good thing, as it helps you catch potential bugs early in the development process.

```rust
println!("You guessed: {guess}");
```

Here the program prints the value of the `guess` variable to the screen. The `{guess}` syntax is a placeholder that will be replaced with the value of the `guess` variable when the program runs.