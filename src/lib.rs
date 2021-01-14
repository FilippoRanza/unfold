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

pub fn unfold<T, F>(func: F, init: T) -> Unfold<T, F>
where
    F: Fn(T) -> T,
    T: Copy,
{
    Unfold::new(func, init)
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

    pub fn to_vector(self, len: usize) -> Vec<T> {
        self.take(len).collect()
    }

    pub fn take_nth(self, index: usize) -> T {
        self.take(index).last().unwrap()
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
