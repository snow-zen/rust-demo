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
pub fn bubble_sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
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
pub fn select_sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
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
pub fn insert_sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    // 找出最小值作为哨兵放在最左边，可避免索引越界检测
    let mut change = false;
    for x in (1..arr.len()).rev() {
        if arr[x] < arr[x - 1] {
            arr.swap(x, x - 1);
            change = true;
        }
    }

    // 如果没有发生过一次交换，则表示数组本身就是有序的
    if !change {
        return;
    }

    // 左移小值进行排序
    for x in 2..arr.len() {
        let mut y = x;
        while arr[y] < arr[y - 1] {
            arr.swap(y, y - 1);
            y -= 1;
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
pub fn merge_sort<T>(arr: &mut [T])
where
    T: Copy + PartialOrd,
{
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

fn merge<T>(left_arr: &[T], right_arr: &[T], intermediate_arr: &mut [T])
where
    T: Copy + PartialOrd,
{
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

/// 快速排序
///
/// ```
/// use algorithm::sort::quick_sort;
/// let mut arr = [3, 2, 1];
/// quick_sort(&mut arr);
/// assert_eq!([1, 2, 3], arr);
/// ```
pub fn quick_sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    if arr.len() <= 1 {
        return;
    }

    do_quick_sort(arr, 0, arr.len() - 1);
}

fn do_quick_sort<T>(arr: &mut [T], start: usize, end: usize)
where
    T: PartialOrd,
{
    if start >= end {
        return;
    }

    let pivot = partition(arr, start, end);
    if let Some(r) = pivot.checked_sub(1) {
        do_quick_sort(arr, start, r);
    }
    if let Some(r) = pivot.checked_add(1) {
        do_quick_sort(arr, r, end);
    }
}

fn partition<T>(arr: &mut [T], start: usize, end: usize) -> usize
where
    T: PartialOrd,
{
    let pivot = start;
    let mut l_idx = start + 1;
    let mut r_idx = end;

    loop {
        while l_idx < arr.len() && arr[l_idx] < arr[pivot] {
            l_idx += 1;
        }
        while r_idx > 0 && arr[r_idx] > arr[pivot] {
            r_idx -= 1;
        }

        if l_idx >= r_idx {
            break;
        }
        arr.swap(l_idx, r_idx);
    }
    arr.swap(pivot, r_idx);
    r_idx
}
