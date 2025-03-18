// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let midpoint = v.len() / 2;

    let r = thread::scope(|scope| {
        let handle1 = scope.spawn(|| {
            let first= &v[..midpoint];
            first.iter().sum::<i32>()});
        let handle2 = scope.spawn(|| {
            let second = &v[midpoint..];
            second.iter().sum::<i32>()});

        let r1 = handle1.join().unwrap();
        let r2 = handle2.join().unwrap();

        r1 + r2
    });

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
