use std::cmp::min;

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

/// 自顶向下归并排序
///
/// ```
/// use algorithm::sort::top_down_merge_sort;
/// let mut arr = [3, 2, 1];
/// top_down_merge_sort(&mut arr);
/// assert_eq!([1, 2, 3], arr);
/// ```
pub fn top_down_merge_sort<T>(arr: &mut [T])
where
    T: Copy + PartialOrd,
{
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    top_down_merge_sort(&mut arr[..mid]);
    top_down_merge_sort(&mut arr[mid..]);

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

/// 自底向上的归并排序
///
/// ```
/// use algorithm::sort::bottom_up_merge_sort;
/// let mut arr = [3, 2, 1];
/// bottom_up_merge_sort(&mut arr);
/// assert_eq!([1, 2, 3], arr);
/// ```
pub fn bottom_up_merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    let mut len = 1;
    while len < arr.len() {
        for start in (0..(arr.len() - len)).step_by(len * 2) {
            let mid = start + len - 1;
            let end = min(start + len * 2 - 1, arr.len() - 1);

            bottom_up_merge(arr, start, mid, end);
        }
        len *= 2;
    }
}

fn bottom_up_merge<T>(arr: &mut [T], start: usize, mid: usize, end: usize)
where
    T: PartialOrd + Copy,
{
    let copy_arr = arr.to_vec();

    let mut i_idx = start;
    let mut j_idx = mid + 1;

    for x in start..=end {
        if i_idx > mid {
            arr[x] = copy_arr[j_idx];
            j_idx += 1;
        } else if j_idx > end {
            arr[x] = copy_arr[i_idx];
            i_idx += 1;
        } else if copy_arr[i_idx] < copy_arr[j_idx] {
            arr[x] = copy_arr[i_idx];
            i_idx += 1;
        } else {
            arr[x] = copy_arr[j_idx];
            j_idx += 1;
        }
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

/// 希尔排序
///
/// ```
/// use algorithm::sort::shell_sort;
/// let mut arr = [3, 2, 1];
/// shell_sort(&mut arr);
/// assert_eq!([1, 2, 3], arr)
/// ```
pub fn shell_sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let mut h = 1;
    while h < arr.len() / 3 {
        h = 3 * h + 1;
    }

    while h >= 1 {
        for x in h..arr.len() {
            let mut y = x;
            while y >= h && arr[y] < arr[y - h] {
                arr.swap(y, y - h);
                y -= h;
            }
        }
        h /= 3;
    }
}
