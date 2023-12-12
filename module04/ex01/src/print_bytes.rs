
use std::ops::FnMut;

fn print_bytes<F: FnMut() -> Option<u8>>(mut f: F) {
    let mut i = 0;
    loop {
        match f() {
            Some(b) => if b == 0 {print!("Y") } else { print!("y") },
            None => break,
        }
        if i % 8 == 7 {
            println!();
        }
        i += 1;
    }
}

fn main() {
    let bytes: &'static [u8] = b"Hello, World!";
    let mut i: usize = 0;
    let mut temp: usize = 0;
    print_bytes(||{
            if i >= bytes.len() {
                return None;
            }
            else {
                let ret = Some(bytes[i] & (128 >> temp));
                temp += 1;
                if temp == 8 {
                    temp = 0;
                    i += 1;
                }
                return ret;
            }
    });
}

// even n/2 ----- odd 3n+1