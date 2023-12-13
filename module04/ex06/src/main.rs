
use std::str::from_utf8;

#[derive(Debug)]
struct FormatError;

type WriteFn<'a> = dyn 'a + FnMut(&str) -> Result<(), FormatError>;

trait Print {
    fn print(&self, write: &mut WriteFn) -> Result<(), FormatError>;
}

impl Print for u32 {
    fn print(&self, f: &mut WriteFn) -> Result<(), FormatError> {

        if *self == 0 {
            return f("0");
        }
        let mut tmp = *self;
        let mut buf = [0; 10];
        let mut i = buf.len();
        while tmp > 0 {
            i -= 1;
            buf[i] = ((tmp % 10) as u8) + b'0';
            tmp /= 10;

        }
        f(from_utf8(&buf[i..10]).unwrap())

    }
}

impl<'a> Print for &'a str {
    fn print(&self, f: &mut WriteFn) -> Result<(), FormatError> {
        return f(*self)
    }
}



// #[test]
// #[cfg(test)]
fn main() {
    1741874287u32.print(&mut |s| { assert_eq!(s, "43"); return Ok(()) });
}
