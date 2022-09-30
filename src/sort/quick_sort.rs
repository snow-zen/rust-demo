/// 快速排序
///
/// 不稳定排序，平均时间复杂度为 O(nlogn)，空间复杂度为 O(logn)。
///
/// # Example
///
/// ```
/// use rust_demo::sort::quick_sort;
/// let mut arr = [2, 1, 3];
/// quick_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 3]);
/// ```
pub fn quick_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    if arr.len() <= 1 {
        return;
    }
    quick(arr, 0, arr.len() - 1);
}

fn quick<T>(arr: &mut [T], start: usize, end: usize)
where
    T: Ord,
{
    if start < end {
        let pivot = partition(arr, start, end);
        if let Some(r) = pivot.checked_sub(1) {
            quick(arr, start, r);
        }
        if let Some(r) = pivot.checked_add(1) {
            quick(arr, r, end);
        }
    }
}

fn partition<T>(arr: &mut [T], start: usize, end: usize) -> usize
where
    T: Ord,
{
    let mid = median(arr, start, start + (end - start + 1) / 2, end);
    arr.swap(start, mid);

    let mut l_idx = start + 1;
    let mut r_idx = end;

    // 假设 arr[l_idx] 是最大的元素
    while arr[l_idx] < arr[start] {
        if l_idx == end {
            arr.swap(l_idx, start);
            return end;
        }
        l_idx += 1;
    }

    // 假设 arr[r_idx] 是最小的元素
    while arr[r_idx] > arr[start] {
        if r_idx == start + 1 {
            return start;
        }
        r_idx -= 1;
    }

    while l_idx < r_idx {
        arr.swap(l_idx, r_idx);
        while arr[l_idx] < arr[start] {
            l_idx += 1;
        }
        while arr[r_idx] > arr[start] {
            r_idx -= 1;
        }
    }

    arr.swap(start, r_idx);
    r_idx
}

fn median<T>(arr: &[T], idx1: usize, idx2: usize, idx3: usize) -> usize
where
    T: Ord,
{
    if arr[idx1] < arr[idx2] {
        if arr[idx2] < arr[idx3] {
            return idx2;
        }
        if arr[idx1] < arr[idx3] {
            return idx3;
        }
        return idx1;
    }
    if arr[idx1] < arr[idx3] {
        return idx1;
    }
    if arr[idx2] < arr[idx3] {
        return idx3;
    }
    idx2
}

#[cfg(test)]
mod tests {
    use super::super::is_sort;
    use super::*;

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = vec![];
        quick_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn reverse_sorted() {
        let mut arr = vec![6, 5, 4, 3, 2, 1];
        quick_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        quick_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn one_element() {
        let mut arr = vec![1];
        quick_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn repeat() {
        let mut arr = vec![2, 2, 1, 3, 3, 1];
        quick_sort(&mut arr);
        assert!(is_sort(&arr));
    }
}
