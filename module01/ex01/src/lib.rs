pub fn min<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a < b {
        a
    } else {
        b
    }
}

#[cfg(test)]
#[test]
fn add_test() {
	let a = 12;
	let b = 18;
	assert!(*min(&a, &b) == a);

}