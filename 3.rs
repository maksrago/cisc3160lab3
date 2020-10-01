// Link to problem
// https://projecteuler.net/problem=3

fn main() {

    // Function to test if a number is prime
    // Will return true if a number is prime

    // This would be optimized by finding the floored square root of the given number
    // As it stands the compuation takes a tremendous amount of time for a large number
    // As it stands the range present in u32 works fine, but going to insanely large numbers within u64 take a long time to computer without optimizing
    fn is_prime(n: u64) -> bool {
	let limit = (n as f64).sqrt() as u64;
	
	for i in 2..=limit {
            if n % i == 0 {
		return false;
            }
	}
	
	true
    }

    let max: u64 = 600851475143;
    let mut largest_prime = 0;

    for x in 0..max {
	if is_prime(x) == true && x != 0 && x != 1 {
	    if max%x == 0 {
		largest_prime = x;
		println!("Prime Factor = {}", largest_prime); // x: i32
	    }
	}
    }

    println!("Largest Prime Factor = {}", largest_prime); // x: i32
}
