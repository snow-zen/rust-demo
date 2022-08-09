#[cfg(test)]
mod tests;

/// 冒泡排序
///
/// ```
/// use algorithm::sort::bubble_sort;
/// let mut arr = [3, 2, 1];
/// bubble_sort(&mut arr);
/// assert_eq!([1, 2, 3], arr);
/// ```
pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    for x in 1..arr.len() {
        let mut sweep = false;
        for y in 0..arr.len() - x {
            if arr[y] > arr[y + 1] {
                arr.swap(y, y + 1);
                sweep = true;
            }
        }

        if !sweep {
            break;
        }
    }
}

/// 选择排序
///
/// ```
/// use algorithm::sort::select_sort;
/// let mut arr = [3, 2, 1];
/// select_sort(&mut arr);
/// assert_eq!([1, 2, 3], arr);
pub fn select_sort<T: PartialOrd>(arr: &mut [T]) {
    for x in 1..arr.len() {
        let mut min_idx = x - 1;
        for y in x..arr.len() {
            if arr[min_idx] > arr[y] {
                min_idx = y;
            }
        }

        if min_idx != (x - 1) {
            arr.swap(min_idx, x - 1);
        }
    }
}

/// 插入排序
///
/// ```
/// use algorithm::sort::{insert_sort, select_sort};
/// let mut arr = [3, 2, 1];
/// select_sort(&mut arr);
/// assert_eq!([1, 2, 3], arr);
/// ```
pub fn insert_sort<T: PartialOrd>(arr: &mut [T]) {
    for x in 1..arr.len() {
        for y in (1..=x).rev() {
            if arr[y] > arr[y - 1] {
                break;
            }
            arr.swap(y, y - 1);
        }
    }
}

/// 归并排序
///
/// ```
/// use algorithm::sort::merge_sort;
/// let mut arr = [3, 2, 1];
/// merge_sort(&mut arr);
/// assert_eq!([1, 2, 3], arr);
/// ```
pub fn merge_sort<T: Copy + PartialOrd>(arr: &mut [T]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut intermediate_arr = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut intermediate_arr);

    arr.copy_from_slice(&intermediate_arr);
}

fn merge<T: Copy + PartialOrd>(left_arr: &[T], right_arr: &[T], intermediate_arr: &mut [T]) {
    let mut l_idx = 0;
    let mut r_idx = 0;
    let mut t_idx = 0;

    while l_idx < left_arr.len() && r_idx < right_arr.len() {
        if left_arr[l_idx] > right_arr[r_idx] {
            intermediate_arr[t_idx] = right_arr[r_idx];
            r_idx += 1;
        } else {
            intermediate_arr[t_idx] = left_arr[l_idx];
            l_idx += 1;
        }
        t_idx += 1;
    }

    if l_idx < left_arr.len() {
        intermediate_arr[t_idx..].copy_from_slice(&left_arr[l_idx..]);
    }

    if r_idx < right_arr.len() {
        intermediate_arr[t_idx..].copy_from_slice(&right_arr[r_idx..]);
    }
}
