# Rust Borrow Checker Example

This repository contains a simple Rust program that demonstrates a common error related to the borrow checker. The program attempts to create both mutable and immutable references to the same variable simultaneously, which is disallowed by Rust to prevent data races and other concurrency issues. 

The `bug.rs` file shows the code that produces a compile-time error. The `bugSolution.rs` demonstrates how to resolve this issue by restructuring the code to ensure that only one type of reference exists at a time.