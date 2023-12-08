fn choose<T>(values: &[T]) -> &T {
    return &values[ftkit::random_number(0..(values.len() as i32)) as usize];
}

fn main() {
    println!("{}", choose(&[1, 2, 3, 4, 6, 4, 6, 7, 98]));
}
