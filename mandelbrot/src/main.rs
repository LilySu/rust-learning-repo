use num::Complex;

struct Complex<T>{ // <T> = for any type T
    re: T,
    im: T,
}

fn main() {
    println!("Hello, world!");
}

fn complex_square_add_loop(c: Complex<f64>){
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        // values of c greater than 0.25 or less than -2 cause z to fly away
        z = z * z + c;
    }
}

// the function's return value: Option<usize>
// means:
// enumerated type - enumerates several variants
// enum Option<T> {
    // None,
    // Some(T),
// }

// Option<usize> indicates whether c is in the Mandlebrot set, if not, how long to iterate to find that out
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    // iterats over the range of integers starting with 0 and up to but not including limit
    for i in 0..limit {
        // limits the number of iterations to try before declaring c to be a member
        if z.norm_sqr() > 4.0 {
            // i is the number of iterations z left the circle of radius 2 
            return Some(i);
        }
        // returns square of z's distance from the origin. 
        z = z * z + c;
    }
    None
    }
}