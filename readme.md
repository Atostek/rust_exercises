# Rust exercises

## Download
Clone this repository
````
git clone https://github.com/Atostek/rust_exercises.git
```
and see the problems in `src/main.rs`

## Usage
You can run individual test cases with 
```
cargo test <test case name>
```
Or you can run all test cases that begin with common prefix with
```
cargo test <common prefix>
```
For example, to run all tests of the problem 2:
```
cargo test test_problem_2
```

Instead of running unit tests, you can also call the code from
the main()-function. In that case you can run it with a command
```
cargo run
```
This is useful for debug-prints, because tests suppress the output.