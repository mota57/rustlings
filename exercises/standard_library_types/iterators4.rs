// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    let mut result = 0_u64;

    if num == 1 || num == 0 {
        result = 1_u64;
    } else if num == 2 {
        result = 2_u64
    } else {
        let a = (2..=num);
        result = a.fold(1_u64, |acc,next|  { acc * next  });
        //acc =1, 1 * 2 = 2
        //acc=2, 2 * 3 = 6
        //acc=6, 6 * 4 = 24

    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }

    #[test]
    fn factorial_of_5() {
        assert_eq!(362_880, factorial(9));
    }
}
