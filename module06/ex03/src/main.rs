use  std::sync::mpsc::sync_channel;
use std::time::Duration;
use std::thread;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: {} <number>", args[0]);
        return;
    }
    
    let head_size = match args[1].parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            println!("{} is not a number", args[1]);
            return;
        }
    };

    let (sender, receiver) = sync_channel::<u32>(1);
    let (sender_input, receiver_input) = sync_channel::<String>(2);
    let (sender_ez, receiver_ez) = sync_channel::<String>(3);
    
    let _handle = thread::spawn (move || {
        loop {
            let input = receiver_input.recv().unwrap();
            print!("philosopher think about: {}", input);
            thread::sleep(Duration::from_millis(5000));
            sender.send(1).unwrap();
        }
    });

    let _handle2 = thread::spawn(move || {
        loop {
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {},
                Err(error) => {
                    println!("Error: {}", error);
                    break;
                }
            };
            sender_ez.send(input).unwrap();
        }
    });
    
    let mut head = 0;
    loop {
        let input = receiver_ez.recv_timeout(Duration::from_millis(100));
        if input.is_ok() {
            if head >= head_size {
                println!("head is full");
                continue;
            }
            let send_clone = sender_input.clone();
            head += 1;
            thread::spawn(move || {
                send_clone.send(input.unwrap()).unwrap();
            });
        }
        let end = receiver.recv_timeout(Duration::from_millis(100));
        if end.is_ok() {
            head -= 1;
        }
    }




}
