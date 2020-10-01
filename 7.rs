// Link to problem
// https://projecteuler.net/problem=7

fn main() {
    
    fn is_prime(n: u64) -> bool {
	let limit = (n as f64).sqrt() as u64;
	
	for i in 2..=limit {
	    if n % i == 0 {
		    return false;
	    }
	}
	
	true
    }

    let prime_location = 100000000;
    let max = prime_location + 1;

    let mut counter = 0;
    let mut prime = 0;
    
    for x in 0..max {
	if is_prime(x) == true && x != 0 && x != 1 {
	    counter = counter + 1;
	    prime = x;
	}

	if counter == 10001 {
	    println!("10,000th Prime Number = {}", prime); // x: i32
	    return;
	}
    }
}
