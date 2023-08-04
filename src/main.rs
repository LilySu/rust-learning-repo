use std::str::FromStr; // bring the standard library trait FromStr
use std::env; // provides the args function

fn main() { // no -> since the function doesn't return a value
    // declare a mutable local variable numbers that initializes it to an empty vector
    // vectors are designed to be gorwn and shrunk dynmaically
    // variable mut is marked to push numbers to the end of it
    // mut allows function body to assign to them
    let mut numbers: Vec<u64> = Vec::new();

    for arg in env::args().skip(1){
        numbers.push(u64::from_str(&arg)// from_str parses our command-line arguments
            .expect("error parsing argument"));
    }

    if numbers.len() == 0{
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d: u64 = numbers[0];
    for m in &numbers[1..]{
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", 
            numbers, d);

//     println!("Hello, world!"); // ! marks macro invocation, not function call

//     let res = gcd(4, 16); // spells out type unsigned 64-bit integer
//     println!("{}", res)
// }

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

// #[test] // Attributes control compiler warning and code sytle checks, include code conditionally
// fn test_gcd(){
//     assert_eq!(gcd(14,15), 1);

//     assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11)
}