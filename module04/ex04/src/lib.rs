struct Increasing<I: Iterator> {
    iter: I,
    last: Option<I::Item>,
}

impl<I: Iterator> Increasing<I> {
    pub fn new<J: IntoIterator<IntoIter = I>>(iter: J) -> Self {
        Self {
            iter: iter.into_iter(),
            last: None,
        }
    }
}

impl<I: Iterator> Iterator for Increasing<I> 
where 
    I::Item: PartialOrd + Clone
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current = self.iter.next()?;

            if let Some(ref last) = self.last {
                if current <= *last {
                    continue ;
                }
            }
            self.last = Some(current.clone());
            break Some(current);
        }
    }
}

#[cfg(test)]
#[test]
fn test_main() {
    let mut iter = Increasing::new([0.4, 0.2, 0.1, 0.2, 0.4, 0.5, 0.4, 0.6]);
    assert_eq!(iter.next(), Some(0.4));
    assert_eq!(iter.next(), Some(0.5));
    assert_eq!(iter.next(), Some(0.6));
    assert_eq!(iter.next(), None);
}