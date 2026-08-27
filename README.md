# Rust Programming Language

> Ref: *The Rust Programming Language* by Steve Klabnik, Carol Nichols, and Chris Krycho.

> All the code are compiled in WSL2, Ubuntu 24.04

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

