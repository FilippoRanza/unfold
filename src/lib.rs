//!
//! # unfold
//!
//! The [unfold](https://en.wikipedia.org/wiki/Fold_(higher-order_function))
//! function takes as input a function *f* and an initial *i* value and returns
//! a list defined as
//! [i, f(i), f(f(i)), ...].
//!
//!   
//! This library defines ```Unfold``` a struct
//! that implements the unfold function as an *endless*
//! iterator.
//!
//!
//! ## Quick Start
//!
//! To use ```Unfold``` is quite simple. The user calls
//! the new function providing a function as first argument
//! and the initial value as second argument.
//! An ```Unfold``` instance can be then used as any iterator,
//! but don't forget: Unfold never ends, it must be stopped.
//!
//! Here an example:
//! ```
//! use unfold::Unfold;
//!
//! // Create a vector containing the first 5 numbers from the Fibonacci
//! // series
//! let fibonacci_numbers: Vec<u64> = Unfold::new(|(a, b)| (b, a + b), (0, 1))
//!                                          .map(|(a, _)| a)
//!                                          .take(5)  //Unfold iterator never stops.
//!                                          .collect();
//! assert_eq!(vec![0, 1, 1, 2, 3], fibonacci_numbers);
//! ```
//!
//!

///
/// This function it is a simple front-end to
/// Unfold::new : allows the user to easily create a
/// new Unfold iterator
pub fn unfold<T, F>(func: F, init: T) -> Unfold<T, F>
where
    F: Fn(T) -> T,
    T: Copy,
{
    Unfold::new(func, init)
}

/// This function create an unfold iterator and collects
/// its first *len* items into a vector
///
/// ```
/// use unfold::unfold_vector;
///
/// let exp_growth = unfold_vector(|x| 2 * x, 1, 10);
///
/// assert_eq!(exp_growth, vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512]);
/// ```
pub fn unfold_vector<T, F>(func: F, init: T, len: usize) -> Vec<T>
where
    F: Fn(T) -> T,
    T: Copy,
{
    unfold(func, init).take(len).collect()
}

///This function create an unfold iterator and returns
/// its *index*-th item.
/// ```
/// use unfold::unfold_nth;
///
/// let n = unfold_nth(|x| x + 1, 0, 10);
/// assert_eq!(n, 9);
///
/// ```
pub fn unfold_nth<T, F>(func: F, init: T, index: usize) -> T
where
    F: Fn(T) -> T,
    T: Copy,
{
    unfold(func, init).take(index).last().unwrap()
}

/// This function create an unfold iterator that stops
/// at *count* iterations.
/// Note this is a standard lazy iterator
/// ```
/// use unfold::unfold_count;
///
/// let mut next_odd = unfold_count(|x| x + 2, 1, 3);
/// assert_eq!(next_odd.next(), Some(1));
/// assert_eq!(next_odd.next(), Some(3));
/// assert_eq!(next_odd.next(), Some(5));
/// assert_eq!(next_odd.next(), None);
///
/// ```
pub fn unfold_count<T, F>(func: F, init: T, count: usize) -> impl Iterator<Item = T>
where
    F: Fn(T) -> T,
    T: Copy,
{
    unfold(func, init).take(count)
}

/// Define an endless unfold iterator
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
    ///
    ///Create a new Unfold instance
    ///```
    /// use unfold::Unfold;
    /// //initial value is 100, the function counts backward
    /// let count_down = Unfold::new(|x| x - 1, 100);
    ///
    ///```
    pub fn new(function: F, init: T) -> Self {
        Self {
            func: function,
            curr: init,
        }
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

    #[test]
    fn test_unfold_nth() {
        let fib = unfold_nth(|(a, b)| (b, a + b), (0, 1), 8).0;
        assert_eq!(fib, 13);
    }

    #[test]
    fn test_unfold_vector() {
        let count = unfold_vector(|x| x + 1, 0, 10);
        let result: Vec<i32> = (0..10).collect();
        assert_eq!(count, result);
    }

    #[test]
    fn test_unfold_count() {
        let mut iter = unfold_count(|x| x + 1, 0, 5);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
    }
}
