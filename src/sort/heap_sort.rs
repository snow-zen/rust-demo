pub fn heap_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    if arr.len() <= 1 {
        return;
    }

    for k in (0..=((arr.len() - 2) / 2)).rev() {
        sink(arr, k);
    }

    for k in (1..arr.len()).rev() {
        arr.swap(0, k);
        sink(&mut arr[..k], 0);
    }
}

fn sink<T>(arr: &mut [T], mut root: usize)
where
    T: Ord,
{
    let end = arr.len() - 1;
    while 2 * root + 1 <= end {
        let mut idx = 2 * root + 1;
        if let Some(right) = arr.get(idx + 1) {
            if arr[idx] < *right {
                idx += 1;
            }
        }

        if arr[idx] > arr[root] {
            arr.swap(idx, root);
        }
        root = idx;
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sort;
    use super::*;

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = vec![];
        heap_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn reverse_sorted() {
        let mut arr = vec![6, 5, 4, 3, 2, 1];
        heap_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        heap_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn one_element() {
        let mut arr = vec![1];
        heap_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn repeat() {
        let mut arr = vec![2, 2, 1, 3, 3, 1];
        heap_sort(&mut arr);
        assert!(is_sort(&arr));
    }
}
