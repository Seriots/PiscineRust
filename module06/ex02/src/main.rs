use std::cell::Cell;

#[derive(Clone, Copy, Debug)]
enum Error {
    Success,
    FileNotFound,
    IsDirectory,
    WriteError,
    ReadError,
}

thread_local! {
    static LAST_ERROR: Cell<Error> = Cell::new(Error::Success);
}

impl Error {
    fn last() -> Self {
        LAST_ERROR.with(|cell| cell.get())
    }
    fn make_last(self) {
        LAST_ERROR.with(|cell| cell.set(self));
    }
}

fn main() {
    let mut handles: Vec<_> = vec![];

    for i in 0..10 {
        let handle = std::thread::spawn(move || {
            let error = Error::make_last(match i {
                0 => Error::FileNotFound,
                1 => Error::IsDirectory,
                2 => Error::WriteError,
                3 => Error::ReadError,
                _ => Error::Success,
            });
            println!("Thread {} generated", i);
            std::thread::sleep(std::time::Duration::from_millis(10));
            println!("Thread {} last error: {:?}", i, Error::last());
        });
        // sleep to make sure the thread is spawned before the next iteration
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
