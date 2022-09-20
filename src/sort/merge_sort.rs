use std::cmp::min;

pub fn td_merge_sort<T>(arr: &mut [T])
where
    T: Ord + Copy,
{
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    td_merge_sort(&mut arr[..mid]);
    td_merge_sort(&mut arr[mid..]);
    td_merge(arr, mid);
}

fn td_merge<T>(arr: &mut [T], mid: usize)
where
    T: Ord + Copy,
{
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();

    let mut left_idx = 0;
    let mut right_idx = 0;

    for x in arr {
        if right_idx >= right_half.len()
            || (left_idx < left_half.len() && left_half[left_idx] < right_half[right_idx])
        {
            *x = left_half[left_idx];
            left_idx += 1;
        } else {
            *x = right_half[right_idx];
            right_idx += 1;
        }
    }
}

pub fn bu_merge_sort<T>(arr: &mut [T])
where
    T: Ord + Copy,
{
    let mut len = 1;
    while len < arr.len() {
        for start in (0..(arr.len() - len)).step_by(len * 2) {
            let mid = start + len - 1;
            let end = min(start + len * 2, arr.len());
            bu_merge(arr, start, mid, end);
        }
        len *= 2;
    }
}

fn bu_merge<T>(arr: &mut [T], start: usize, mid: usize, end: usize)
where
    T: Ord + Copy,
{
    let temp_copy = arr.to_vec();
    let mut left_idx = start;
    let mut right_idx = mid + 1;

    for x in start..end {
        if right_idx >= end || (left_idx <= mid && temp_copy[left_idx] < temp_copy[right_idx]) {
            arr[x] = temp_copy[left_idx];
            left_idx += 1;
        } else {
            arr[x] = temp_copy[right_idx];
            right_idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sort;
    use super::*;

    #[test]
    fn empty() {
        let mut arr1: Vec<i32> = vec![];
        td_merge_sort(&mut arr1);
        assert!(is_sort(&arr1));

        let mut arr2: Vec<i32> = vec![];
        bu_merge_sort(&mut arr2);
        assert!(is_sort(&arr2));
    }

    #[test]
    fn reverse_sorted() {
        let mut arr1 = vec![6, 5, 3, 2, 1];
        td_merge_sort(&mut arr1);
        assert!(is_sort(&arr1));

        let mut arr2: Vec<i32> = vec![6, 5, 3, 2, 1];
        bu_merge_sort(&mut arr2);
        assert!(is_sort(&arr2));
    }

    #[test]
    fn sorted() {
        let mut arr1 = vec![1, 2, 3, 4, 5, 6];
        td_merge_sort(&mut arr1);
        assert!(is_sort(&arr1));

        let mut arr2 = vec![1, 2, 3, 4, 5, 6];
        bu_merge_sort(&mut arr2);
        assert!(is_sort(&arr2));
    }

    #[test]
    fn one_element() {
        let mut arr1 = vec![1];
        td_merge_sort(&mut arr1);
        assert!(is_sort(&arr1));

        let mut arr2 = vec![1];
        bu_merge_sort(&mut arr2);
        assert!(is_sort(&arr2));
    }

    #[test]
    fn repeat() {
        let mut arr1 = vec![2, 2, 1, 3, 3, 1];
        td_merge_sort(&mut arr1);
        assert!(is_sort(&arr1));

        let mut arr2 = vec![2, 2, 1, 3, 3, 1];
        bu_merge_sort(&mut arr2);
        assert!(is_sort(&arr2));
    }
}
