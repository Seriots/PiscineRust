use std::str::FromStr;

pub fn sum_of_odds(s: &str) -> u32 {
    s.split_whitespace().filter_map(|x| x.parse::<u32>().ok()).filter(|x| x % 2 == 1).sum()
}

pub fn create_pairs<T: FromStr>(s: &str) -> Vec<(&str, T)> {
    s.lines()
        .filter_map(|x| x.split_once(":"))
        .map(|(k, v)| (k.trim(), v.trim()))
        .filter_map(|(k, v)| v.parse::<T>().ok().map(|v| (k, v)))
        .collect()
}

#[test]
fn sum_test() {
    let sum = sum_of_odds("hey 20 test 3\t9 4 5, 1 hello");
    assert_eq!(sum, 13);
}

#[cfg(test)]
#[test]
fn test() {

    let input = "\
    first: 1
    second 2
       \t third   : 3
    fourth
    fifth  : 43\t
    \tsixth
    ";
    
    let v: Vec<(&str, u32)> = create_pairs(input);
    
    assert_eq!(
        v,
        [
            ("first", 1),
            ("third", 3),
            ("fifth", 43),
        ]
    );
}