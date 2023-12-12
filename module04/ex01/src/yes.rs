
use std::ops::FnOnce;

fn yes<F: FnOnce() -> String>(f: F) -> ! {
    let s = f();
    loop {
        println!("{}", s);
    }

}

fn main() {
    yes(|| {
        return "YyY".to_string();
    });
}
