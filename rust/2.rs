// Link to problem
// https://projecteuler.net/problem=2

fn main() {
    let mut x = 0; // first starting term in Fibonacci Sequence
    let mut y = 1; // second starting term in Fibonacci Sequence
    let mut z = x + y;
    let mut sum = 0; // this will track the sum of even valued terms
    
    while z < 4000000 {
	z = x + y;
	x = y; // set the first term to be equal to the old second term
	y = z; // set the second term to be equal to the new term

	if z%2 == 0 {
	    sum += z;
	}
    }
    println!("sum = {}", sum);
}
