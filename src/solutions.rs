#![allow(unused_variables, dead_code)]  // Do not warn about unimplemented functions

/// Rust exercices
/// Implement functions by following instructions and unit tests.
/// 
/// Download:
///     git clone https://github.com/Atostek/rust_exercises.git
/// 
/// You can run individual test cases with 
///     cargo test <test function>
/// For example: 
///     cargo test test_problem_1_new
/// Or you can run all test cases that begin with a common prefix
///     cargo test <common prefix>
/// For example, to run all tests of the problem 1:
///     cargo test test_problem_1
/// 
/// Tip: You can debug print variables with:
///          dbg!(some_variable);
///      Unit test runs are not the most convenient for debug printing, because
///      tests also print other information. To see clear output, you can call the
///      functions from the main()-function. For that case, run the main-function with:
///          cargo run


// -------------


/// Problem 1
/// 
/// Write a non-empty linked list that stores signed integers (i32). 
/// You may use or modify the struct 'NonEmptyLinkedList' below. 
/// Implement the methods in `impl` block by following the documentation and the unit tests.

/// Node of a non-empty linked list. The head of this list is the first item.
/// Both terms "list" and "node" are used to refer this struct.
#[derive(PartialEq, Debug)]  // Allows tests to check equality and print the struct if it fails.
struct NonEmptyLinkedList {
    num: i32,
    next: Option<Box<NonEmptyLinkedList>>
}

impl NonEmptyLinkedList {
    /// Create a new list with one element
    /// Tip: That is, initialize a node with num=item next = None
    fn new(item: i32) -> NonEmptyLinkedList {
        NonEmptyLinkedList {num: item, next: None}
    }

    /// Add an element to the front of the list with O(1) computation time.
    /// Note: This method does not modify the list/node, but rather it creates a new one.
    /// Note: The input parameter 'self' is a value, (and not a reference '&self'). 
    ///       This means you have ownership of the list, and you can move it freely.
    /// Tip: While there are many ways to make a new node, the most straight-forward way 
    ///      is to move the current node inside the newly created one.
    fn push_front(self, item: i32) -> NonEmptyLinkedList {
        NonEmptyLinkedList {num: item, next: Some(Box::new(self))}
    }

    /// Overwrite the first element in the list
    fn set_front(&mut self, new_value: i32) {
        self.num = new_value;
    }

    /// Convert the list into a string.
    /// This part may be more difficult, feel free to skip if stuck.
    ///
    /// Tip: You can format text into end of a string with:
    ///          use std::fmt::Write;
    ///          write!(&mut my_string, "a formatted number: {}", number).unwrap();
    /// Tip: You can iterate the node structure with "while let" -syntax:
    ///          while let Some(next_node) = &node.next {
    ///              ...
    ///              node = next_node;
    ///          }
    /// Tip: Rust string acts like a vector. You can remove the last character with .pop()
    fn to_string(&self) -> String {
        use std::fmt::Write;

        let mut node = self;
        let mut output = format!("[");

        while let Some(next_node) = &node.next {
            write!(output, "{}, ", node.num).unwrap();
            node = next_node;
        }
        write!(output, "{}]", node.num).unwrap();

        output
    }
}

#[test]
fn test_problem_1_new () {
    let l = NonEmptyLinkedList::new(1);
    assert_eq!(l, NonEmptyLinkedList {num: 1, next: None});
}
#[test]
fn test_problem_1_push_front () {
    let l = NonEmptyLinkedList::new(1).push_front(2);
    assert_eq!(l, NonEmptyLinkedList {num: 2, next: Some(Box::new(NonEmptyLinkedList {num: 1, next: None}))});
}
#[test]
fn test_problem_1_set_front () {
    let mut l = NonEmptyLinkedList::new(1).push_front(2);
    l.set_front(4);
    assert_eq!(l, NonEmptyLinkedList {num: 4, next: Some(Box::new(NonEmptyLinkedList {num: 1, next: None}))});
}
#[test]
fn test_problem_1_to_string () {
    let l = NonEmptyLinkedList::new(1).push_front(2).push_front(3);
    assert_eq!(l.to_string(), "[3, 2, 1]");
}


// -------------


/// Problem 2
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
/// - match-expression is useful for this problem 
///     - https://doc.rust-lang.org/rust-by-example/flow_control/match.html
/// - You can iterate over characters of a string with:
///     for c in input.chars() {...}
fn matching_parenthesis(input: &str) -> bool {
    let mut stack = Vec::new();
    for c in input.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                let Some(prev) = stack.pop() else { return false; };
                match (prev, c) {
                    ('(', ')') | ('[', ']') | ('{', '}') => (),
                    _ => return false,
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}

#[test]
fn test_problem_2 () {
    assert_eq!(matching_parenthesis("(asdf)"), true);
    assert_eq!(matching_parenthesis(")asdf("), false);
    assert_eq!(matching_parenthesis("(asdf))"), false);
    assert_eq!(matching_parenthesis("([)]"), false);
    assert_eq!(matching_parenthesis("([]{})"), true);
    assert_eq!(matching_parenthesis("[{}](){[]()}"), true);
}


// -------------


/// Problem 3 a)
/// 
/// Write a function that checks if given positive number is a prime.
/// 
/// Tip: Number 1 is a prime. To check if N is a prime, check that no whole number 
///      between 2..sqrt(N) divides N. 
fn is_prime(n: u64) -> bool {
    let sqrt = (n as f64).sqrt().ceil() as u64;
    (2..sqrt).all(|i| n % i != 0 )
}

#[test]
fn test_problem_3_a() {
    assert_eq!(is_prime(1), true);
    assert_eq!(is_prime(13), true);
    assert_eq!(is_prime(15), false);
}

/// Problem 3 b)
/// 
/// Make a function that finds all primes between two numbers using `is_prime()` from the
/// previous part. Use iterators instead of a for-loop. (Part-c depends on this part to 
/// be solved with iterators.)
///
/// Tips:
/// - To create iterator over range of numbers, use `(start..end)`. Then take primes, 
///   and collect them into a vector.
/// - For the reference of the syntax, this was a code example from the slides
///       let even_squares = numbers.iter()
///           .filter(|&x| x % 2 == 0)
///           .map(|x| x * x)
///           .collect::<Vec<_>>();
/// - For more, see documentation on iterator:
///     https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
fn primes_between(start: u64, end: u64) -> Vec<u64> {
    (start..end).filter(|n| is_prime(*n)).collect()
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
/// Take your previous implementation, and parallellize it with multithreading! 
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
fn primes_between_parallel(start: u64, end: u64) -> Vec<u64> {
    use rayon::prelude::*;
    (start..end).into_par_iter().filter(|n| is_prime(*n)).collect()
}

#[test]
fn test_problem_3_c() {
    assert_eq!(
        primes_between_parallel(10_u64.pow(14), 10_u64.pow(14) + 100), 
        [100000000000031, 100000000000067, 100000000000097, 100000000000099]
    );
}


// -------------


/// Problem 4
/// 
/// This following problem is more difficult than others, and it is presented here only if
/// more challenge is wanted.
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
/// Tips:
/// - If you need to take n:th value from iterator, use `.nth(n)`. This consumes the values up to n.
/// - If you need to append a value into iterator, you can use `.chain(std::iter::once(some_value))`
/// - This function returns Option. So you can use `?` to unwrap functions that return an option.
/// - Iterator documentation: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
fn substr(s: &str, begin: usize, end: usize) -> Option<&str> {
    let last_idx = std::iter::once(s.len());
    let mut itr = s.char_indices().map(|(n, _)| n).chain(last_idx);
    let begin_byte = itr.nth(begin)?;
    let end_byte = if begin >= end {
        begin_byte
    } else {
        itr.nth(end-begin-1)?
    };
    Some(&s[begin_byte..end_byte])
}

const TEXT: &str = "tekstiä🙂";
#[test]
fn test_problem_4_empty() {
    assert_eq!(Some(""), substr(TEXT, 2, 2));
}
#[test]
fn test_problem_4_ascii() {
    assert_eq!(Some("ekst"), substr(TEXT, 1, 5));
}
#[test]
fn test_problem_4_scandinavian() {
    assert_eq!(Some("tekstiä"), substr(TEXT, 0, 7));
}
#[test]
fn test_problem_4_emojis() {
    assert_eq!(Some("ä🙂"), substr(TEXT, 6, 8));
}
#[test]
fn test_problem_4_out_of_bounds() {
    assert_eq!(None, substr(TEXT, 6, 9));
    assert_eq!(None, substr(TEXT, 9, 6));
}


// --------------------


// You can test your functions by calling them from here, and running 'cargo run'.
fn main() {

}
