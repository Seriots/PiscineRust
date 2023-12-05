pub fn strpcmp(query: &[u8], pattern: &[u8]) -> bool {
    let mut i = 0;
    while i < pattern.len() && pattern[i] != b'*' {
        if i >= query.len() || pattern[i] != query[i] {
            return false;
        }
        i += 1;
    }
    if i == pattern.len() {
        return i == query.len();
    }
    i += 1;
    while i < pattern.len() && pattern[i] == b'*' {
        i += 1;
    }
    if i == pattern.len() {
        return true;
    }
    let mut j = 0;
    while j < query.len() {
        if strpcmp(&query[j..], &pattern[i..]) {
            return true;
        }
        j += 1;
    }
    false
}

#[cfg(test)]
#[test]
fn random_test() {
	assert!(strpcmp(b"abc", b"abc"));

	assert!(strpcmp(b"abcd", b"ab*"));
	assert!(!strpcmp(b"cab", b"ab*"));

	assert!(strpcmp(b"dcab", b"*ab"));
	assert!(!strpcmp(b"abc", b"*ab"));

	assert!(strpcmp(b"ab000cd", b"ab*cd"));
	assert!(strpcmp(b"abcd", b"ab*cd"));

	assert!(strpcmp(b"", b"****"));
}