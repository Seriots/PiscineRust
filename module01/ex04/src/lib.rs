pub fn sort_boxes(boxes: &mut [[u32; 2]]) {
    let mut i = 0;
    while i < boxes.len() {
        let mut j = i + 1;

        while j < boxes.len() {
            if boxes[i][0] <= boxes[j][0] && boxes[i][1] <= boxes[j][1] {
                boxes.swap(i, j);
            }
            j += 1;
        }
        assert!(i == 0 || (boxes[i][0] <= boxes[i - 1][0] && boxes[i][1] <= boxes[i - 1][1]));
        i += 1;
    }
}

#[cfg(test)]
#[test]
fn test_boxes() {
    let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
    sort_boxes(&mut boxes);
    assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
    
}

#[cfg(test)]
#[test]
#[should_panic]
fn test_impossible() {
    let mut boxes = [[1, 2], [2, 1]];
    sort_boxes(&mut boxes);
}