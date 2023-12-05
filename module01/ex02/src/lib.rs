#![allow(dead_code)]

const pub fn color_name(color: & [u8; 3]) -> &'static str {
    match color {
        &[0, 0, 0] => "pure black",
        &[255, 255, 255] => "pure white",
        &[255, 0, 0] => "pure red",
        &[0, 255, 0] => "pure green",
        &[0, 0, 255] => "pure blue",
        &[128, 128, 128] => "pure grey",
        &[r, g, b] if r <= 31 && g <= 31 && b <= 31 => "almost black",
        &[r, g, b] if r > 128 && g <= 127 && b <= 127 => "redish",
        &[r, g, b] if r <= 127 && g > 128 && b <= 127 => "greenish",
        &[r, g, b] if r <= 127 && g <= 127 && b > 128 => "blueish",
        _ => "unknown",
    }
}

#[cfg(test)]
#[test]
fn test_lifetimes() {
    let name_of_the_best_color;

    {   
        let the_best_color = [42, 42, 42];
        name_of_the_best_color = color_name(&the_best_color);
    }

    assert_eq!(name_of_the_best_color, "unknown");
}