use unfold;

fn unfold_sqrt(n: f64) -> Option<f64> {
    if n < 0.0 {
        None
    } else if n == 1.0 || n == 0.0 {
        // 0 and 1 are  special cases
        Some(n)
    } else {
        // this is Newton's  method to find root in non linear function
        // applied to the function  f(x) = x² - n. 
        // The positive root, the one found by this function, is the square root of n
        unfold::unfold_count(|x| ((x * x) + n) / (2.0 * x), n, 100)
            .take_while(|x| ((x * x) - n).abs() > 1e-8)
            .last()
    }
}

fn main() {
    println!("sqrt(100) = {} ", unfold_sqrt(100.0).unwrap());
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_square_root() {
        for i in 0..20 {
            let n = i as f64;
            let res = unfold_sqrt(n * n).unwrap();
            // Newton's method has a quadratic convergence
            // so  the last itation with an error larger then
            // 1e-8 has an error in the order of 1e-4
            assert!((res - n).abs() < 1e-4);
        }
    }

}

