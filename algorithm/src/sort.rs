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
