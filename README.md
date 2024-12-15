# Rust Index Out of Bounds Bug

This repository demonstrates a common runtime error in Rust: accessing an element in a vector using an index that is out of bounds. The code attempts to access an element at index 10, while the vector only contains elements at indices 0, 1, and 2.  This results in a panic.

The `bug.rs` file contains the buggy code. The `bugSolution.rs` file shows how to fix the error using bounds checking.

## How to Run

1. Clone the repository.
2. Navigate to the repository's directory.
3. Compile and run `bug.rs` (you'll see a panic).
4. Compile and run `bugSolution.rs` (this will execute without errors).