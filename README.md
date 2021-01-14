# unfold
A simple unfold implementation in Rust

[unfold](https://en.wikipedia.org/wiki/Fold_(higher-order_function)) let you 
create an iterator that, staring from a given initial value, 
applies a given function to the current state, store the result for
the next iteration, and return the current state. 

Unfold defines an endless iterator: you must stop it *by hand*, like in the 
example. 

## Example
```
use unfold::Unfold;

// Create a vector containing the first 5 numbers from the Fibonacci
// series
let fibonacci_numbers: Vec<u64> = Unfold::new(|(a, b)| (b, a + b), (0, 1))
                                         .map(|(a, _)| a)
                                         .take(5)  //Unfold iterator never stops.
                                         .collect();
assert_eq!(vec![0, 1, 1, 2, 3], fibonacci_numbers);
```
