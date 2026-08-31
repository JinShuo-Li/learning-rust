# Rust Programming Language

> Ref: *The Rust Programming Language* by Steve Klabnik, Carol Nichols, and Chris Krycho.

> All the code is compiled on WSL2, Ubuntu 24.04.

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

We also need a *linker*, which is a program that Rust uses to join its compiled outputs into one file. On Linux, users should generally install GCC or Clang.

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

Now we can initialize the first Rust project:

```
cargo new ch01_startup
```

If you don't want to initialize it with `git`, you can run:

```
cargo new ch01_startup --vcs none
```

Then you can see a new directory appear in your working directory. This is the new Rust project you just made.

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

`Cargo` also provides a command called `cargo check` that *quickly* checks your code to make sure it compiles but doesn't produce an executable.

There are also other useful `cargo` commands; we *may* introduce them in later chapters.

### Hello, World!

In the Rust project we just made, we can see a Rust source file named `main.rs` in `src/`, which contains:

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

These lines define a function named main. The main function is special: it is always the first code that runs in every executable Rust program. If there were parameters, they would go inside the `()`; and the function body is wrapped in curly brackets `{}`.

```rust
println!("Hello, World!");
```

This line prints text to the screen. Here are three more things we need to know:

1. `println!` calls a **Macro**; it is a way to write code that generates code to extend Rust syntax.
2. `"Hello, World!"` is a `string`; we pass this string as an argument to `println!`, and it is printed to the screen.
3. We end the line with a semicolon `;`, which indicates that this expression is over and the next one is ready to begin.

We can also compile a `.rs` file directly with `rustc`, without `cargo`. Run:

```shell
rustc main.rs
```

Then run `./main` to see the output.

But we still recommend using `cargo`.

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

Here, the program uses the `std::io` library, which is a standard library that provides you with a number of useful features, including the ability to accept user input.

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

Then:

```rust
.expect("Failed to read line");
```

Here the program handles potential failure with the `Result` type. The `read_line()` method returns a `Result` value, which can be either `Ok` or `Err`. If the result is `Err`, the program will panic and print the message "Failed to read line". If the result is `Ok`, the program will continue executing.

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

In Rust, warnings are not errors, and they do not prevent the program from running. Rust is designed to be strict and safe, so it warns you about potential problems in your code even when they would not stop compilation. This is a good thing, as it helps you catch potential bugs early in the development process.

```rust
println!("You guessed: {guess}");
```

Here the program prints the value of the `guess` variable to the screen. The `{guess}` syntax is a placeholder that will be replaced with the value of the `guess` variable when the program runs.

### Generating a Random Number

Now we can implement a new version of the guessing game, which generates a random number as the secret. The source code is as follows:

```rust
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

To use the `rand` crate, we need to add it to the `Cargo.toml` file:

```toml
[dependencies]
rand = "0.8.5"
```

If we want to update the `rand` crate to the latest version, we can run the following command:

```shell
cargo update
```

Now we can compile and run the program, and it will generate a random number between 1 and 100 as the secret.

```rust
use rand::Rng;
```

First we add the line `use rand::Rng;` to the top of the file. This line tells Rust that we want to use the `Rng` trait from the `rand` crate. The `Rng` trait defines methods that random number generators implement, and we need it to generate random numbers.

```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```

Here we call the `gen_range()` method on the random number generator returned by `rand::thread_rng()`. The `gen_range()` method takes a range as an argument and returns a random number within that range. In this case, we are generating a random number between 1 and 100 (inclusive). If we want to generate a random number between 1 and 100 (exclusive), we can use the range `1..100` instead.

### Comparing the Guess with the Secret Number

Now we can compare the user's guess with the secret number. The source code is as follows:

```rust
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

First, we add the line:

```rust
use std::cmp::Ordering;
```

Bringing a type called `std::cmp::Ordering` into scope. The `Ordering` type is an enum that has three variants: `Less`, `Greater`, and `Equal`. We will use this type to compare the user's guess with the secret number.

Then we add the following line:

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

This line converts the `String` to a `u32` by trimming whitespace and parsing the string as a number. If the conversion fails, it will panic with the message "Please type a number!". The `trim()` method removes any whitespace from the beginning and end of the string, and the `parse()` method attempts to convert the string to a number. The `expect()` method is used to handle any potential errors that may occur during the conversion process.

Finally, we add the following code to compare the user's guess with the secret number:

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

The `cmp()` method compares the user's guess with the secret number and returns an `Ordering` value. We use a `match` expression to handle each possible outcome: if the guess is less than the secret number, we print "Too small!"; if it is greater, we print "Too big!"; and if it is equal, we print "You win!".

### Allowing Multiple Guesses with Looping

Now we can allow the user to make multiple guesses until they guess the correct number. The source code is as follows:

```rust
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(_num) => _num,
            Err(_num) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

Here the program uses a `loop` to allow the user to make multiple guesses:

```rust
loop {
    // code
}
```

The `loop` runs the code inside it over and over until the user guesses the correct number. It never stops on its own: it runs forever until it is stopped with a `break` statement or `Ctrl+C`.

We also modify the code so that the game will quit when the user wins by adding a `break` statement inside the `Ordering::Equal` arm of the `match` expression:

```rust
Ordering::Equal => {
    println!("You win!");
    break;
}
```

Now the game will continue to prompt the user for guesses until they guess the correct number, at which point it will print "You win!" and exit the loop.

We also modify the code to handle invalid input by using a `match` expression to check if the user's input can be parsed as a number:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(_num) => _num,
    Err(_num) => continue,
};
```

The `match` expression checks if the `parse()` method returns an `Ok` or an `Err`. If it returns `Ok`, we assign the parsed number to the `guess` variable. If it returns `Err`, we use the `continue` statement to skip the rest of the loop and prompt the user for another guess.

## Chapter 3: Common Programming Concepts

This chapter covers concepts that appear in every programming language and how they work in Rust.

### Variables and Mutability

As mentioned in the previous chapter, **variables are immutable** by default. When a variable is immutable, once a value is bound to a name, you can't change that value. For example, if you write the following lines into a `.rs` file and compile it using `rustc`, you will see:

```rust
fn main() {
    let x = 5;
    println!("The value of x is {x}");

    // mut the value of x
    x = 6;
    println!("The value of x is {x}")
}
```
The line starting with `let` means that the program creates a new variable `x`. The line starting with `x =` tries to assign the value `6` to the variable `x`.

If you compile the code above, you will see:

```shell
error[E0384]: cannot assign twice to immutable variable `x`
 --> test.rs:5:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
...
5 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0384`.
```

Following the compiler's suggestion, we can make a variable mutable by adding `mut` in front of the variable name as we did in Chapter 2.

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is {x}");

    // mut the value of x
    x = 6;
    println!("The value of x is {x}")
}
```

#### Declaring Constants

Constants are **immutable**, and can be declared in any scope, including the global scope, which makes them useful for global parameters. For example:

```rust
const THRESHOLD: i32 = 1024;
```

Constants are valid for the entire time a program runs, within the scope in which they were declared.

#### Shadowing

Let's first look at an example:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is {x}");
}
```

run `cargo run` and we will see:

```shell
   Compiling ch03_basics v0.1.0 (/home/lijs/tmp/rustlearning/ch03_basics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `/home/lijs/tmp/rustlearning/target/debug/ch03_basics`
The value of x in the inner scope is: 12
The value of x is 6
```

Shadowing is different from marking a variable as `mut` because we create a new variable with the same name. Also, even with `mut` we can't mutate a variable's type, but shadowing can.

### Data Types

Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so that it knows how to work with that data. We'll look at two data type subsets: scalar and compound.

**Keep in mind that Rust is a statically typed language**, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it. In cases when many types are possible, such as when we converted a `String` to a numeric type using parse in Chapter 2, we must add a type annotation like `u32` or `i32`.

#### Scalar Types

Scalar types represent a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

**Integer Types**

An integer is a number without a fractional component. In Rust, we have several integer types:

| Length | Signed | Unsigned |
|--------|--------|----------|
|  8-bit | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
|128-bit | i128   | u128     |
|Architecture-dependent | isize | usize |

Each signed n-bit integer can store numbers from $-2^{n-1}$ to $2^{n-1} - 1$, and each unsigned n-bit integer can store numbers from $0$ to $2^n - 1$.

Additionally, the `isize` and `usize` types depend on the architecture of the computer your program is running on. On a 64-bit architecture, these types will be 64 bits in size, and on a 32-bit architecture, they will be 32 bits in size.

**Floating-Point Types**

Rust also has two primitive types for floating-point numbers: `f32` and `f64`. The default type is `f64`, which is generally the best choice in terms of speed and precision.

**Floating-point numbers are represented according to IEEE-754 Standard.**

- Numeric Operations

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```

**Boolean Type**

As in most other programming languages, the Boolean type in Rust has two possible values: `true` and `false`. The Boolean type is specified using the `bool` keyword.

```rust
fn main() {
    let t = true;
    let f: bool = false;
}
```


**The Character Type**

Rust's `char` type is the language's most primitive alphabetic type. It represents a single character and is specified using the `char` keyword. Rust's `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a wide range of characters from different languages and symbol sets.

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ';
}
```
