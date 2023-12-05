
pub fn largest_group<'a>(haystack: &'a [u32], needle: &[u32]) -> &'a [u32] {
    let mut size = 0;
    let mut offset = 0;
    let mut i = 0;

    while i < haystack.len() {
        let mut j = 0;
        while needle.contains(&haystack[i + j]) {
            j += 1;
            if i + j >= haystack.len() {
                break;
            }
        }
        if j > size {
            let mut nop = false;
            for k in 0..needle.len() {
                if !haystack[i..i + j].contains(&needle[k]) {
                    nop = true;
                    break;
                }
            }
            if nop == false {
                size = j;
                offset = i;
            }
        }
        i += 1;
    }
    return &haystack[offset..offset + size];
}

#[test]
#[cfg(test)]
fn test_lifetimes() {
    let haystack = [1, 2, 3, 2, 1];
    let result;

    {
        let needle = [2, 3];
        result = largest_group(&haystack, &needle);
    }

    assert_eq!(result, &[2, 3, 2]);

}

#[test]
#[cfg(test)]
fn test_largest_group() {
    assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
    assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5]), &[5, 5]);
    assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[]), &[]);
    assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[4, 1]), &[]);
}