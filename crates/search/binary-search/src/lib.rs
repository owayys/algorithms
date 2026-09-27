use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum BinarySearchError {
    NotFound,
}

impl fmt::Display for BinarySearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinarySearchError::NotFound => {
                write!(f, "Target element was not found in array.")
            }
        }
    }
}

pub fn binary_search_iterative(array: &[u64], target: u64) -> Result<usize, BinarySearchError> {
    let mut low: usize = 0;
    let mut high: usize = array.len();

    while low < high {
        let mid = low + (high - low) / 2;

        match array.get(mid) {
            Some(value) => match value.cmp(&target) {
                std::cmp::Ordering::Equal => return Ok(mid),
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => high = mid,
            },
            None => break,
        }
    }

    Err(BinarySearchError::NotFound)
}

pub fn search(
    array: &[u64],
    target: u64,
    low: usize,
    high: usize,
) -> Result<usize, BinarySearchError> {
    if low >= high {
        return Err(BinarySearchError::NotFound);
    }

    let mid = low + (high - low) / 2;

    match array.get(mid) {
        Some(value) => match value.cmp(&target) {
            std::cmp::Ordering::Equal => Ok(mid),
            std::cmp::Ordering::Less => search(array, target, mid + 1, high),
            std::cmp::Ordering::Greater => search(array, target, low, mid),
        },
        None => Err(BinarySearchError::NotFound),
    }
}

pub fn binary_search_recursive(array: &[u64], target: u64) -> Result<usize, BinarySearchError> {
    if array.is_empty() {
        return Err(BinarySearchError::NotFound);
    }

    search(array, target, 0, array.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_slice_not_found() {
        let array: [u64; 0] = [];
        assert_eq!(
            binary_search_iterative(&array, 5),
            Err(BinarySearchError::NotFound)
        );
        assert_eq!(
            binary_search_recursive(&array, 5),
            Err(BinarySearchError::NotFound)
        );
    }

    #[test]
    fn finds_first_and_last_element() {
        let array = [1, 3, 5, 7, 9];
        assert_eq!(binary_search_iterative(&array, 1), Ok(0));
        assert_eq!(binary_search_iterative(&array, 9), Ok(4));
        assert_eq!(binary_search_recursive(&array, 1), Ok(0));
        assert_eq!(binary_search_recursive(&array, 9), Ok(4));
    }

    #[test]
    fn target_not_present() {
        let array = [10, 20, 30];
        assert_eq!(
            binary_search_iterative(&array, 25),
            Err(BinarySearchError::NotFound)
        );
        assert_eq!(
            binary_search_recursive(&array, 25),
            Err(BinarySearchError::NotFound)
        );
    }

    #[test]
    fn handles_duplicates() {
        let array = [1, 2, 2, 2, 3];
        assert_eq!(array[binary_search_iterative(&array, 2).unwrap()], 2);
        assert_eq!(array[binary_search_recursive(&array, 2).unwrap()], 2);
    }

    #[test]
    fn no_underflow_when_target_below_all() {
        let array = [5, 10, 15, 20];
        assert_eq!(
            binary_search_iterative(&array, 1),
            Err(BinarySearchError::NotFound)
        );
        assert_eq!(
            binary_search_recursive(&array, 1),
            Err(BinarySearchError::NotFound)
        );
    }
}
