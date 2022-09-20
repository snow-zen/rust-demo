pub fn bubble_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    if arr.is_empty() {
        return;
    }

    let mut sorted = false;
    let mut end = arr.len();

    while !sorted {
        sorted = true;
        for i in 0..end - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        end -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sort;
    use super::*;

    #[test]
    fn reverse_sorted() {
        let mut arr = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn empty() {
        let mut arr: Vec<usize> = vec![];
        bubble_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn one_element() {
        let mut arr = vec![1];
        bubble_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn repeat() {
        let mut arr = vec![2, 2, 1, 3, 3, 1];
        bubble_sort(&mut arr);
        assert!(is_sort(&arr));
    }
}
