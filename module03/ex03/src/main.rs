use std::fmt::Debug;

trait FortyTwo {
    fn forty_two() -> Self;
}

impl FortyTwo for u32 {
    fn forty_two() -> Self {
        42
    }
}

impl FortyTwo for String {
    fn forty_two() -> Self {
        String::from("FortyTwo")
    }
}

fn print_forty_two<T: Debug + FortyTwo>() {
    let a = T::forty_two();
    println!("{a:?}");
}

fn main() {
    print_forty_two::<u32>();
    print_forty_two::<String>();
}
