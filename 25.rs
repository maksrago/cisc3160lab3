// Link to problem
// https://projecteuler.net/problem=25

fn main() {
    let mut x = 0; // first starting term in Fibonacci Sequence
    let mut y = 1; // second starting term in Fibonacci Sequence
    let mut z = x + y;
    
    for i in 0..10000 {
	z = x + y;
	x = y; // set the first term to be equal to the old second term
	y = z; // set the second term to be equal to the new term

	if (i + 1) == 2 {
	    z = 1;
	    y = 1;
	}

	if y >= 1000 {
	    println!("F({}) = {}", i + 1, y);
	    return;
	}
    }
}
