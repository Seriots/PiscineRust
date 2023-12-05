fn my_contains(list: &Vec<i32>, elem: i32) -> bool {
    for e in 0..list.len() {
        if list[e] == elem {
            return true;
        }
    }
    false
}

pub fn deduplicate(list: &mut Vec<i32>) {
    let mut witness = Vec::new();
    let mut end = false;
    let mut i = 0;

    while end == false {
        end = true;
        for elem in i..list.len() {
            if !my_contains(&witness, list[elem]) {
                witness.push(list[elem]);
            }
            else
            {
                list.remove(elem);
                end = false;
                i = elem;
                println!("{:?}", list);
                break;
            }
        }
    }
}

// pub fn deduplicate(list: &mut Vec<i32>) {
//     let mut i = 0;
//     while i < list.len() {
//         let mut j = list.len() - 1;
//         while j > i {
//             if list[i] == list[j] {
//                 list.remove(j);
//             }
//             j -= 1;
//         }
//         i += 1;
//     }
// }

#[cfg(test)]
#[test]
fn test_dedup() {
    let mut v = vec![1, 2, 2, 3, 2, 4, 3];
    deduplicate(&mut v);
    assert_eq!(v, [1, 2, 3, 4]);
}