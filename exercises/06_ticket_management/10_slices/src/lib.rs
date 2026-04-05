// TODO: Define a function named `sum` that takes a reference to a slice of `u32` and returns the sum of all
//  elements in the slice.

/* // my wacky method thats generic af based on compiler feedback
fn sum<T>(input: &[T]) -> u32 where u32: for<'a> std::iter::Sum<&'a T>{
    let x = input.iter().sum();
    return x
}
*/

// the simple way
fn sum(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let v = vec![];
        assert_eq!(sum(&v), 0);
    }

    #[test]
    fn one_element() {
        let v = vec![1];
        assert_eq!(sum(&v), 1);
    }

    #[test]
    fn multiple_elements() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }

    #[test]
    fn array_slice() {
        let v = [1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }
}
