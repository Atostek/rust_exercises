#![allow(unused_variables, dead_code)]  // Do not warn about unimplemented functions

/// Rust exercices
/// Implement functions by following instructions and unit tests.
/// 
/// Download:
///     git clone https://github.com/Atostek/rust_exercises.git
/// 
/// You can run individual test cases with 
///     cargo test <test case name>
/// Or you can run all test cases that begin with common prefix
///     cargo test <common prefix>
/// For example, to run all tests of the problem 2:
///     cargo test test_problem_2
/// 
/// Instead of running unit tests, you can also call the code from
/// the main()-function. In that case you can run it with a command
///     cargo run
/// This is useful for debug-prints, because tests suppress the output.




/// Problem1
/// 
/// Write a function that checks if all parenthesis in a string match each other.
/// There can be three types of parenthesis () [] {}, and they can be nested. 
/// String can also contain other characters, but they can be ignored.
/// See tests below for examples.
/// 
/// Tips:
/// - This problem is easiest to solve with a stack
/// - See std vector: https://doc.rust-lang.org/std/vec/struct.Vec.html
///     - Vec::new(), .push(), .pop();
/// - match expression is useful for this problem 
///     - https://doc.rust-lang.org/rust-by-example/flow_control/match.html
/// - Iterate over (unicode) characters of a string with input.chars()
fn matching_parenthesis(input: &str) -> bool {
    unimplemented!()
}

#[test]
fn test_problem_1 () {
    assert_eq!(matching_parenthesis("(asdf)"), true);
    assert_eq!(matching_parenthesis(")asdf("), false);
    assert_eq!(matching_parenthesis("(asdf))"), false);
    assert_eq!(matching_parenthesis("([)]"), false);
    assert_eq!(matching_parenthesis("([]{})"), true);
    assert_eq!(matching_parenthesis("[{}](){[]()}"), true);
}


// -------------


/// Problem 2
/// 
/// Write a non-empty linked list that stores signed integers (i32). 
/// You may use (or modify) the struct below. 
/// Implement the methods in `impl` block by following documentation and unit tests below.
/// 
/// Note:
/// - The line below, #[derive(..)], is an attribute, and it auto-implements partial equality and 
///   debug-printability.
#[derive(PartialEq, Debug)]
struct NonEmptyLinkedList {
    num: i32,
    next: Option<Box<NonEmptyLinkedList>>
}

impl NonEmptyLinkedList {
    /// Creates a new list with one element.
    fn new(item: i32) -> NonEmptyLinkedList {
        unimplemented!()
    }

    /// Adds an element to the end list with O(1) computation time.
    /// Note that this consumes the calling list and returns a new list.
    fn push(self, item: i32) -> NonEmptyLinkedList {
        unimplemented!()
    }

    /// Converts the list to a String. 
    fn to_string(&self) -> String {
        unimplemented!()
    }
    
    /// Overwrites the last element in the list
    fn set_last(&mut self, new_value: i32) {
        unimplemented!()
    }
}

#[test]
fn test_problem_2_new () {
    let l = NonEmptyLinkedList::new(1);
    assert_eq!(l, NonEmptyLinkedList {num: 1, next: None});
}
#[test]
fn test_problem_2_push () {
    // test push()
    let l = NonEmptyLinkedList::new(1).push(2);
    assert_eq!(l, NonEmptyLinkedList {num: 2, next: Some(Box::new(NonEmptyLinkedList {num: 1, next: None}))});
}
#[test]
fn test_problem_2_to_string () {
    let l = NonEmptyLinkedList::new(1).push(2).push(3);
    assert_eq!(l.to_string(), "[1, 2, 3]");
}
#[test]
fn test_problem_2_set_last () {
    // test set_last()
    let mut l = NonEmptyLinkedList::new(1).push(2);
    l.set_last(4);
    assert_eq!(l, NonEmptyLinkedList {num: 4, next: Some(Box::new(NonEmptyLinkedList {num: 1, next: None}))});
}


// -------------


/// Problem 3 a)
/// 
/// Write a function that checks if given positive number is a prime.
/// You can use test_prime() to test the algorithm.
/// 
/// Tips:
/// - number 1 is prime
/// - To check if N is prime, just check that no whole number between 2..sqrt(N) divides N. 
fn is_prime(n: u64) -> bool {
    unimplemented!()
}

#[test]
fn test_problem_3_a() {
    assert_eq!(is_prime(1), true);
    assert_eq!(is_prime(13), true);
    assert_eq!(is_prime(15), false);
}

/// Problem 3 b)
/// 
/// Make a function that finds all primes betwee two numbers using previous `is_prime()`-function.
/// Use iterators instead of for-loop. That is, start with a range `start..end`, then filter only 
/// those numbers that are primes, and then collect them into a vector.
/// 
/// Tips
/// - range a..b is also an iterator. Thus you can call iterator methods directly on it.
///     - Though you need to add parenthesis around it like so: `(1..9).map(|x| x*x)`
/// - Documentation:
///     - range: https://doc.rust-lang.org/stable/std/ops/struct.Range.html
///     - iterator: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
fn primes_between(start: u64, end: u64) -> Vec<u64> {
    unimplemented!()
}

#[test]
fn test_problem_3_b() {
    assert_eq!(
        primes_between(10_u64.pow(14), 10_u64.pow(14) + 100), 
        [100000000000031, 100000000000067, 100000000000097, 100000000000099]
    );
}

/// Problem 3 c)
/// 
/// Take your previous implementation, and make it utilize multithreading!
/// Use a third party library called "rayon": https://docs.rs/rayon/latest/rayon/iter/index.html
/// 
/// Tips:
/// - To add new external library, run command: 
///     `cargo add rayon`
/// - Or add it manually by putting following line under "[dependencies]" in Cargo.toml: 
///     rayon = "1.8.0"
/// - Import rayon traits into scope. This extends interface of iterables with parallel iterators
///     use rayon::prelude::*;
/// - A range x..y can be turned into parallel iterator with 
///     (x..y).into_par_iter()
/// - For additional experimentation
///     - If you are interested on performance, you can measure execution time with:
///         let now = std::time::Instant::now();
///         // run some code
///         println!("Elapsed {} ms", now.elapsed().as_millis());
///     - Tests suppresses output, so you need to run this from main() to see elapsed time
///     - If you want to run with optimizations, use
///         cargo run --release
fn primes_between_parallel(start: u64, end: u64) -> Vec<u64> {
    unimplemented!()
}

#[test]
fn test_problem_3_c() {
    assert_eq!(
        primes_between_parallel(10_u64.pow(14), 10_u64.pow(14) + 100), 
        [100000000000031, 100000000000067, 100000000000097, 100000000000099]
    );
}


// --------------------


/// Problem 4
/// 
/// Note: This following problem may be a bit more difficult than others, and it is presented here 
/// only if there's a need for more. It refers to slices, which were not coverd on the day 1. 
/// A slice is a reference to range of data. For example a string slice `"text"[1..3]` evaluates 
/// to "ex".
/// 
/// Following function returns a substring of given input string slice.
/// However, it does not take UTF-8 (unicode) inputs into account. Thus, if input text contains 
/// emojis or scandinavian letters, then the function will panic. Modify the function so that it 
/// handles unicode correctly. Your task is to find proper values for `begin_byte` and `end_byte`.
/// Also, return None if the index is out of bound.
/// 
/// Use method `s.char_indices()` to obtain iterator over valid unicode characters with indices.
/// https://doc.rust-lang.org/stable/std/primitive.str.html#method.char_indices
/// 
/// If you want an extra challenge, avoid a second redundant iteration. That is, save the iterator 
/// into variable, progress it to find `begin_byte`, then progress it some more to find `end_byte`.
/// 
/// Tips:
/// - If you need to take n:th value from iterator, use `.nth(n)`. This consumes the values up to n.
/// - If you need to append a value into iterator, you can use `.chain(std::iter::once(some_value))`
/// - This function returns Option. So you can use `?` to unrwrap functions that return an option.
/// - Iterator documentation: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
fn substr(s: &str, begin: usize, end: usize) -> Option<&str> {
    let begin_byte = begin;
    let end_byte = end;

    Some(&s[begin_byte..end_byte])
}

const TEXT: &str = "tekstiä🙂";
#[test]
fn test_problem_4_empty() {
    assert_eq!(Some(""),        substr(TEXT, 2, 2));
}
#[test]
fn test_problem_4_ascii() {
    assert_eq!(Some("ekst"),    substr(TEXT, 1, 5));
}
#[test]
fn test_problem_4_scandinavian() {
    assert_eq!(Some("tekstiä"), substr(TEXT, 0, 7));
}
#[test]
fn test_problem_4_emojis() {
    assert_eq!(Some("ä🙂"),     substr(TEXT, 6, 8));
}
#[test]
fn test_problem_4_out_of_bounds() {
    assert_eq!(None,            substr(TEXT, 6, 9));
    assert_eq!(None,            substr(TEXT, 9, 6));
}


// --------------------


fn main() {


}
