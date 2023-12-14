use std::io::{Stdout, StdoutLock, Stdin, StdinLock};
use std::io::{Write, Read, stdin, stdout};
use std::fs::File;


pub fn to_str(buffer: &[u8]) -> String {
    let mut str: String = String::new();
    for i in buffer {
        str.push(*i as char);
    }
    str
}

fn main() {
    let mut v: Vec<File> = Vec::new();

    std::env::args().skip(1).for_each(|arg| {
        File::create(&arg).map(|file| {
            v.push(file);
        }).unwrap_or_else(|e| {
            println!("error: {:?}", e);
        });
    });

    let mut buf :[u8; 4096] = [0; 4096];

    loop {
        match stdin().read(&mut buf) {
            Ok(0) => break,
            Ok(count) => {
                match stdout().write_all(&buf[..count]) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("error: {:?}", e);
                    }
                };
                for each in &mut v {
                    match each.write_all(&buf[..count]) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("error: {:?}", e);
                        }
                    };
                }
            },
            Err(e) => {
                println!("error: {:?}", e);
            }
        };

    }
}
