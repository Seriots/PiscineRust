
use  std::sync::atomic::AtomicU8;
use  std::sync::atomic::Ordering;

#[derive(Debug, PartialEq, Eq)]
struct Unique(u8);

impl Unique {
    fn new_unique() -> u8 {
        static COUNTER: AtomicU8 = AtomicU8::new(0);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
    pub fn new() -> Self {
        Unique(Unique::new_unique())
    }
}

impl Clone for Unique {
    fn clone(&self) -> Self {
        Unique::new()
    }
}

fn main() {
    let a = Unique::new();
    let b = Unique::new();
    let c = Unique::new();

    println!("{a:?}");
    println!("{b:?}");
    println!("{c:?}");

    let d = a.clone();
    let e = c.clone();

    println!("{d:?}");
    println!("{e:?}");
}