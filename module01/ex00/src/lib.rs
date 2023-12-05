#![allow(dead_code)]

pub fn add(a: &i32, b: i32) -> i32 {
	*a + b
}

pub fn add_assign(a: &mut i32, b: i32) {
	*a += b;
}

#[cfg(test)]
#[test]
fn add_test() {
	let mut a = 18;
	assert!(add(&1, 2) == 3);
	assert!(add(&a, 2) == 20);
	add_assign(&mut a, 2);
	assert!(a == 20);
	add_assign(&mut a, 10);
	assert!(a == 30);

}