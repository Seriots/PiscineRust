fn max(a: usize, b: usize) -> usize {
	if a > b
		{a}
	else
		{b}
}


pub fn big_add(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut vec_a: Vec<u8> = Vec::new();
    let mut vec_b: Vec<u8> = Vec::new();

    if a.is_empty() || b.is_empty() {
        panic!("Empty slices are not allowed");
    }
    for &x in a {
        if !x.is_ascii_digit(){
            panic!("Invalid digit {}", x);
        }
        vec_a.push(x - 48);
    }
    for &x in b {
        if !x.is_ascii_digit(){
            panic!("Invalid digit {}", x);
        }
        vec_b.push(x - 48);
    }

    let mut carry: u8 = 0;
    let mut result: Vec<u8> = Vec::new();

    vec_a.reverse();
    vec_b.reverse();

    for i in 0..max(vec_a.len(), vec_b.len()) {
        let sum = carry + if i < vec_a.len() { vec_a[i] } else {0} + if i < vec_b.len() { vec_b[i] } else {0};
        result.push(sum % 10 + 48);
        carry = sum / 10;
    }
    if carry > 0 {
        result.push(carry + 48);
    }
    result.reverse();
    return result;
}

#[cfg(test)]
#[test]
fn test_big_add() {
    assert_eq!(big_add(b"123", b"321"), b"444");
    assert_eq!(big_add(b"999", b"1"), b"1000");
    assert_eq!(big_add(b"999", b"999"), b"1998");
    assert_eq!(big_add(b"1234567890", b"9876543210"), b"11111111100");
}

#[cfg(test)]
#[test]
#[should_panic]
fn test_big_add_empty_a() {
    big_add(b"123", b"");
}

#[cfg(test)]
#[test]
#[should_panic]
fn test_big_add_empty_b() {
    big_add(b"", b"123");
}

#[cfg(test)]
#[test]
#[should_panic]
fn test_big_add_no_ascii() {
    big_add(b"541a", b"123");
}

