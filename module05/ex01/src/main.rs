use std::io::{Stdout, StdoutLock, Stdin, StdinLock};
use std::io::{Write, Read, stdin, stdout};


fn to_str(buffer: &[u8]) -> String {
    let mut str: String = String::new();
    for i in buffer {
        str.push(*i as char);
    }
    str
}

fn main() {
    let mut buf :[u8; 4096] = [0; 4096];

    loop {
        match stdin().read(&mut buf) {
            Ok(0) => break,
            Ok(count) => {
                stdout().write(&buf[..count]).unwrap();
            },
            Err(e) => {
                println!("error: {:?}", e);
            }
        };

    }
}
