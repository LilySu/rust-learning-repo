// write cargo 42 56 in command line to pass arguments to the program

// Rusts's rules for ownership and references are key to memory management and safe concurrency.

use std::str::FromStr; // bring the standard library trait FromStr
use std::env; // provides the args function
// the args function that std:env provides returns an iterator, producing each argument on demand.
// iterators include a broad selection of methods

// if main returns at all, the program finished successfully
fn main() { // no -> since the function doesn't return a value
    // declare a mutable local variable numbers that initializes it to an empty vector
    // vectors are designed to be gorwn and shrunk dynmaically
    // variable mut is marked to push numbers to the end of it
    // mut allows function body to assign to them
    // initializes an empty vector, like a list
    let mut numbers = Vec::new();
    // use a for loop to process command-line arguments, set "arg to each argument
    for arg in env::args().skip(1){
        numbers.push(u64::from_str(&arg)// from_str parses our command-line arguments
            .expect("error parsing argument"));
    }

    if numbers.len() == 0{
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }
    // check vector has at least one element and exit the program with an error if it doesn't
    // d is used as its running value, updated to be the greatest common devisior and is mutable so it can be assigned 
    let mut d: u64 = numbers[0];
    for m in &numbers[1..]{ // & borrows a reference to the vector's elements from the second onward.
        d = gcd(d, *m); // * is the next value we want to pass onto the gcd
    }
    // substitutes formatted versions of the arguments and writes to standard output stream
    println!("The greatest common divisor of {:?} is {}", 
            numbers, d);

//     println!("Hello, world!"); // ! marks macro invocation, not function call

//     let res = gcd(4, 16); // spells out type unsigned 64-bit integer
//     println!("{}", res)
// }

// #[test] // Attributes control compiler warning and code sytle checks, include code conditionally
// fn test_gcd(){
//     assert_eq!(gcd(14,15), 1);

//     assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0); // if not true, terminates the program with helpful message
    while m != 0 { // requires brackets not parentesis
        if m < n {
            let t: u64 = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n // return
}