# Cryptopals, cryptographic solutions in Rust 🦀✨
This project contains solutions to the original set of the [Matasano Cryptopals](https://cryptopals.com/) cryptographic challenges.

## Introduction

This is a serie of 56 technical challenges around software programming and cryptography made by some people at Matasano.  
The tools folder contains implementations of several standard cryptographic protocols and utilities: do not use them in production.  
The solutions provided here are implemented in [Rust](https://www.rust-lang.org/).


## Requirements

Before you can run the Rust solutions for the Cryptopals challenges, ensure that you have Rust and Cargo installed on your machine. Follow these steps to install Rust using `rustup`:

### Step 1: Install Rust using rustup

If you're running macOS, Linux, or another Unix-like OS, it's recommended to use `rustup` for Rust installation. Open your terminal and run the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Step 2: Verify the Installation

After the installation is complete, open a new terminal and run the following command to verify that Rust is installed:

```bash
rustc --version
```

## Progress

| Set | Challenge | Status |
| --- | --------- | ------ |
| 1   | c01       | ✅     |
| 1   | c02       | ✅     |
| 1   | c03       | ✅     |
| 1   | c04       | ✅      |
| 1   | c05       | ✅      |
| 1   | c06       | ✅     |
| 1   | c07       | 🔨      |
| ... | ...       |       |

## Running Tests

To run all tests, you can use the following command:

```bash
cd challenges
cargo test
```

If you want to run a specific test, you can use the challenge name. For example, to run the test associated with c02, you can use:

```bash
cd challenges
cargo test c02
```
