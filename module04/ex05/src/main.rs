
struct Groups<'a, F> {
    s: &'a str,
    f: F,
}

impl<'a, F: FnMut(char) -> bool> Groups<'a, F> {
    pub fn new(s: &'a str, f: F) -> Self {
        Self {
            s,
            f,
        }
    }
}

impl<'a, F: FnMut(char) -> bool> Iterator for Groups<'a, F> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.s = self.s.trim_start_matches(|c| !(self.f)(c));
        let mut splits = self.s.splitn(2, |c| !(self.f)(c));
        let ret = splits.next();
        self.s = splits.next().unwrap_or("");

        if ret == Some("") {
            None
        } else {
            ret
        }
    }
}

impl<'a, F: FnMut(char) -> bool> DoubleEndedIterator for Groups<'a, F> {

    fn next_back(&mut self) -> Option<Self::Item> {
        self.s = self.s.trim_end_matches(|c| !(self.f)(c));
        let mut splits = self.s.rsplitn(2, |c| !(self.f)(c));
        let ret = splits.next();
        self.s = splits.next().unwrap_or("");

        if ret == Some("") {
            None
        } else {
            ret
        }
    }
}

fn main() {
    let mut groups = Groups::new("hello,\tworld oiuytrfdfvb uytrfghj", char::is_alphabetic);

    assert_eq!(groups.next(), Some("hello"));
    assert_eq!(groups.next(), Some("world"));
    assert_eq!(groups.next(), Some("oiuytrfdfvb"));
    assert_eq!(groups.next(), Some("uytrfghj"));
    assert_eq!(groups.next(), None);

    let mut groups2 = Groups::new("  abc\t def,test", char::is_alphabetic);

    assert_eq!(groups2.next(), Some("abc"));
    assert_eq!(groups2.next_back(), Some("test"));
    assert_eq!(groups2.next_back(), Some("def"));
    println!("{:?}", groups2.next());
    assert_eq!(groups2.next(), None);
}
