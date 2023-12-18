use std::sync::{Mutex, Arc};
use std::thread::spawn;
use std::io::Write;
use std::thread::JoinHandle;

struct Logger<W> {
    buffer: Box<[u8]>,
    writer: W,
}

impl<W: Write> Logger<W> {

    pub fn last_index(&self) -> usize {
        let mut index = 0;
        for (i, &byte) in self.buffer.iter().enumerate() {
            if byte == 0 {
                index = i;
                break;
            }
        }
        return index;
    }

    pub fn log(&mut self, message: &str) -> std::io::Result<()> {
        static NEWLINE: &str = "\n";
        let message = (message.to_owned() + NEWLINE).as_bytes().to_owned();
        let mut buf_index = self.last_index();

        for &byte in message.iter() {
            if buf_index == self.buffer.len() {
                self.flush();
                buf_index = 0;
            }
            self.buffer[buf_index] = byte;
            buf_index += 1;
        }

        return Ok(());
    }

    pub fn flush(&mut self) {
        self.writer.write_all(&self.buffer[..]).unwrap();
        self.buffer = vec![0; self.buffer.len()].into_boxed_slice();
    }
}

impl<W> Logger<W> {
    pub fn new(threshold: usize, writer: W) -> Self {
        Self {
            buffer: vec![0; threshold].into_boxed_slice(),
            writer,
        }
    }
}


fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<_> = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut logger = Logger::new(40, std::io::stdout());
            let mut turn = 0;
            loop {
                logger.log(format!("hello {} from {}!", turn, i).as_str()).unwrap();
                let _locked = counter.lock().unwrap();
                logger.flush();
                turn+=1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
