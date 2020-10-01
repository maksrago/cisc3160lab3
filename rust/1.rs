// Link to problem
// https://projecteuler.net/problem=1

fn main() {
    let mut sum = 0;
    
    for x in 0..1000 {
	let y = x.to_string().chars().last().unwrap();

	if (x % 3 == 0 && x != 0) || (y == '0' || y == '5') {
	    sum += x;
	}	
    }

    println!("sum = {}", sum);
}
