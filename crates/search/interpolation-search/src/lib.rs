use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum InterpolationSearchError {
    NotFound,
}

impl fmt::Display for InterpolationSearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpolationSearchError::NotFound => {
                write!(f, "Target element was not found in array.")
            }
        }
    }
}

pub fn interpolation_search(array: &[u64], target: u64) -> Result<usize, InterpolationSearchError> {
    if array.is_empty() {
        return Err(InterpolationSearchError::NotFound);
    }

    let mut low: usize = 0;
    let mut high: usize = array.len() - 1;

    while low <= high && target >= array[low] && target <= array[high] {
        if array[low] == array[high] {
            if array[low] == target {
                return Ok(low);
            }
            break;
        }

        let pos_f64 = low as f64
            + ((target - array[low]) as f64 / (array[high] - array[low]) as f64)
                * (high - low) as f64;

        let pos = pos_f64 as usize; // truncation toward 0 -> floor yay

        let current_val = match array.get(pos) {
            Some(&val) => val,
            None => break,
        };

        match current_val.cmp(&target) {
            std::cmp::Ordering::Equal => return Ok(pos),
            std::cmp::Ordering::Less => {
                low = pos + 1;
            }
            std::cmp::Ordering::Greater => {
                if pos == 0 {
                    break;
                }
                high = pos - 1;
            }
        }
    }

    Err(InterpolationSearchError::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_slice_not_found() {
        let array: [u64; 0] = [];
        assert_eq!(
            interpolation_search(&array, 5),
            Err(InterpolationSearchError::NotFound)
        );
    }

    #[test]
    fn finds_first_and_last_element() {
        let array = [1, 3, 5, 7, 9];
        assert_eq!(interpolation_search(&array, 1), Ok(0));
        assert_eq!(interpolation_search(&array, 9), Ok(4));
    }

    #[test]
    fn target_not_present() {
        let array = [10, 20, 30, 40, 50];
        assert_eq!(
            interpolation_search(&array, 25),
            Err(InterpolationSearchError::NotFound)
        );
    }

    #[test]
    fn handles_duplicates() {
        let array = [1, 2, 2, 2, 3];
        assert_eq!(array[interpolation_search(&array, 2).unwrap()], 2);
    }

    #[test]
    fn no_division_by_zero_when_all_elements_equal() {
        let array = [7, 7, 7, 7, 7];
        assert_eq!(array[interpolation_search(&array, 7).unwrap()], 7);
        assert_eq!(
            interpolation_search(&array, 3),
            Err(InterpolationSearchError::NotFound)
        );
    }

    #[test]
    fn target_outside_range() {
        let array = [10, 20, 30, 40, 50];
        assert_eq!(
            interpolation_search(&array, 1),
            Err(InterpolationSearchError::NotFound)
        );
        assert_eq!(
            interpolation_search(&array, 99),
            Err(InterpolationSearchError::NotFound)
        );
    }
}
