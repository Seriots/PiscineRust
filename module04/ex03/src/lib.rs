pub struct Fibs {
    curr: Option<u32>,
    next: Option<u32>,
}

impl Fibs {
    fn new(first: u32, second: u32) -> Self {
        Self {
            curr: Some(first),
            next: Some(second),
        }
    }
}

impl Iterator for Fibs {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        if current == None || self.curr == None {
            return current;
        }
        if let Some(v) = self.curr.unwrap().checked_add(current.unwrap()) {
            self.next = Some(v);
        } else {
            self.next = None;    
        }
        return current
    }
}

fn even_fibs_bellow_1000() -> u32 {
    Fibs::new(0, 1) .take_while(|&fib| fib < 1000).filter(|x| x % 2 == 0).sum()
}

#[test]
fn first_fibs() {
    let mut fibs = Fibs::new(0, 1);

    assert_eq!(fibs.next(), Some(0));
    assert_eq!(fibs.next(), Some(1));
    assert_eq!(fibs.next(), Some(1));
    assert_eq!(fibs.next(), Some(2));
    assert_eq!(fibs.next(), Some(3));
    assert_eq!(fibs.next(), Some(5));
}

#[test]
fn fibs_count() {
    assert_eq!(Fibs::new(0, 1).count(), 48);
}

#[test]
fn test() {
    let mut count = 0;
    for fib in Fibs::new(0, 1) {
        if fib >= 1000 {
            break;
        }
        if fib % 2 == 0 {
            count += fib;
        }
    }

    assert_eq!(count, 798);
}

#[test]
fn test_event_fibs_bellow_1000() {
    assert_eq!(even_fibs_bellow_1000(), 798);
}