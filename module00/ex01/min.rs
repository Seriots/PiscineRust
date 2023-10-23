fn min(a: i32, b: i32) -> i32 {
	if a > b
		{b}
	else
		{a}
}

fn main() {
	println!("5 and 8 : {}", min(5, 8));
	println!("14 and 987 : {}", min(14, 987));
	println!("55854 and 98745 : {}", min(55854, 98745));
	println!("0 and 1 : {}", min(0, 1));
	println!("1 and 1 : {}", min(1, 1));
}