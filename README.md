This repository demonstrates a common error when working with raw pointers and vectors in Rust. The `bug.rs` file contains code that leads to undefined behavior because it modifies a vector's contents via a raw pointer after the vector's capacity might have changed. The `bugSolution.rs` file shows how to avoid this error by using safer methods.