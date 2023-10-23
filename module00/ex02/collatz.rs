fn collatz(start: u32) {
	
	let mut n = start;
	println!("{}", n);
	while n != 1
	{
		if n % 2 == 0
		{
			n = n / 2;
		}
		else
		{
			n = 3 * n + 1;
		}
		println!("{}", n);
	} 
}

fn main() {
	println!("---9874----");
	collatz(9874);
	println!("---12----");
	collatz(12);
	println!("---7----");
	collatz(7);
}