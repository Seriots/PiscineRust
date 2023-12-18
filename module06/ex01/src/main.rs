use std::sync::Mutex;
// use std::thread::spawn;
use std::io::Write;

struct Logger<W> {
    buffer: Box<[u8]>,
    writer: W,
}

impl<W: io::Write> Logger<W> {
    pub fn log(&mut self, message: &str) -> io::Result<()> {
        
    }
    pub fn flush(&mut self);
}

impl<W> Logger<W> {
    pub fn new(threshold: usize, writer: W) -> Self {
        Self {
            buffer: Box::new(vec![0; threshold]),
            writer,
        }
    }
}


fn main() {
    println!("Hello, world!");
}
