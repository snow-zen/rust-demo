pub fn selection_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    let len = arr.len();
    for left_idx in 0..len {
        let mut smallest_idx = left_idx;
        for right_idx in (left_idx + 1)..len {
            if arr[left_idx] > arr[right_idx] {
                smallest_idx = right_idx;
            }
        }
        arr.swap(smallest_idx, left_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sort;
    use super::*;

    #[test]
    fn reverse_sorted() {
        let mut arr = vec![6, 5, 4, 3, 2, 1];
        selection_sort(&mut arr);
        assert!(is_sort(&arr))
    }

    #[test]
    fn sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        selection_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = vec![];
        selection_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn one_element() {
        let mut arr = vec![1];
        selection_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn repeat() {
        let mut arr = vec![2, 2, 1, 3, 3, 1];
        selection_sort(&mut arr);
        assert!(is_sort(&arr));
    }
}
