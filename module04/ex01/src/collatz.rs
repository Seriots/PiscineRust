
use std::ops::Fn;

fn collatz<F: Fn(u32)>(start: u32, f: F) {
    if start == 1 {
        f(start);
        return;
    }
    else if start % 2 == 0 {
        collatz(start / 2, f);
    } else {
        f(start);
        collatz(3 * start + 1, f);
    }

}

fn main() {
    collatz(11, |x| {
        for _ in 0..x {
            print!("Y");
        }
        println!();
    });
}

// even n/2 ----- odd 3n+1