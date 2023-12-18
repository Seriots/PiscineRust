
use std::cell::Cell;

pub fn swap_u32(a: &Cell<u32>, b: &Cell<u32>) {
    let c = a.get();
    a.set(b.get());
    b.set(c);
}

pub fn swap_string(a: &Cell<String>, b: &Cell<String>) {
    a.swap(b);
}



#[cfg(test)]
#[test]
fn test_swap_u32() {
    let a = Cell::new(1);
    let b = Cell::new(2);
    swap_u32(&a, &b);
    assert_eq!(a.get(), 2);
    assert_eq!(b.get(), 1);
}


#[cfg(test)]
#[test]
fn test_swap_string() {

    let a = Cell::new("ABC".into());
    let b = Cell::new("DEF".into());

    swap_string(&a, &b);

    assert_eq!(a.into_inner(), "DEF");
    assert_eq!(b.into_inner(), "ABC");
}