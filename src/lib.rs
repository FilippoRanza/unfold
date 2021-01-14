pub struct Unfold<T, F>
where
    F: Fn(T) -> T,
    T: Copy,
{
    curr: T,
    func: F,
}

impl<T, F> Unfold<T, F>
where
    F: Fn(T) -> T,
    T: Copy,
{
    pub fn new(func: F, curr: T) -> Self {
        Self { func, curr }
    }
}

impl<T, F> Iterator for Unfold<T, F>
where
    F: Fn(T) -> T,
    T: Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.curr;
        self.curr = (self.func)(self.curr);
        Some(tmp)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fibonacci() {
        let fib = Unfold::new(|(a, b)| (b, a + b), (0, 1))
            .map(|(a, _)| a)
            .take(8)
            .last()
            .unwrap();
        assert_eq!(fib, 13);
    }
}
