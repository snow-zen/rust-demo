use std::cmp::min;

#[cfg(test)]
mod tests;

pub trait Sort<T> {
    fn sort(arr: &mut [T]);
}

pub struct Bubble;

impl<T: PartialOrd> Sort<T> for Bubble {
    /// 冒泡排序
    ///
    /// ```
    /// use algorithm::sort::{Bubble, Sort};
    /// let mut arr = [3, 2, 1];
    /// Bubble::sort(&mut arr);
    /// assert_eq!([1, 2, 3], arr);
    /// ```
    fn sort(arr: &mut [T]) {
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
}

pub struct Select;

impl<T: PartialOrd> Sort<T> for Select {
    /// 选择排序
    ///
    /// ```
    /// use algorithm::sort::{Select, Sort};
    /// let mut arr = [3, 2, 1];
    /// Select::sort(&mut arr);
    /// assert_eq!([1, 2, 3], arr);
    /// ```
    fn sort(arr: &mut [T]) {
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
}

pub struct Insert;

impl<T: PartialOrd> Sort<T> for Insert {
    /// 插入排序
    ///
    /// ```
    /// use algorithm::sort::{Insert, Sort};
    /// let mut arr = [3, 2, 1];
    /// Insert::sort(&mut arr);
    /// assert_eq!([1, 2, 3], arr);
    /// ```
    fn sort(arr: &mut [T]) {
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
}

pub struct TdMerge;

impl TdMerge {
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
}

impl<T: PartialOrd + Copy> Sort<T> for TdMerge {
    /// 自顶向下归并排序
    ///
    /// ```
    /// use algorithm::sort::{TdMerge, Sort};
    /// let mut arr = [3, 2, 1];
    /// TdMerge::sort(&mut arr);
    /// assert_eq!([1, 2, 3], arr);
    /// ```
    fn sort(arr: &mut [T]) {
        if arr.len() < 7 {
            Insert::sort(arr);
            return;
        }

        let mid = arr.len() / 2;
        if mid == 0 {
            return;
        }

        TdMerge::sort(&mut arr[..mid]);
        TdMerge::sort(&mut arr[mid..]);

        if arr[mid - 1] <= arr[mid] {
            // 左边区域的最大值小于等于右边区域的最小值时，表示本身已经有序。
            return;
        }

        let mut intermediate_arr = arr.to_vec();
        TdMerge::merge(&arr[..mid], &arr[mid..], &mut intermediate_arr);

        arr.copy_from_slice(&intermediate_arr);
    }
}

pub struct BuMerge;

impl BuMerge {
    fn merge<T>(arr: &mut [T], start: usize, mid: usize, end: usize)
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
}

impl<T: PartialOrd + Copy> Sort<T> for BuMerge {
    /// 自底向上的归并排序
    ///
    /// ```
    /// use algorithm::sort::{BuMerge, Sort};
    /// let mut arr = [3, 2, 1];
    /// BuMerge::sort(&mut arr);
    /// assert_eq!([1, 2, 3], arr);
    /// ```
    fn sort(arr: &mut [T]) {
        let mut len = 1;
        while len < arr.len() {
            for start in (0..(arr.len() - len)).step_by(len * 2) {
                let mid = start + len - 1;
                let end = min(start + len * 2 - 1, arr.len() - 1);

                BuMerge::merge(arr, start, mid, end);
            }
            len *= 2;
        }
    }
}

pub struct Quick;

impl Quick {
    fn do_quick_sort<T>(arr: &mut [T], start: usize, end: usize)
    where
        T: PartialOrd,
    {
        if start >= end {
            return;
        }

        if end - start + 1 <= 8 {
            Insert::sort(&mut arr[start..=end]);
            return;
        }

        let pivot = Quick::partition(arr, start, end);
        if let Some(r) = pivot.checked_sub(1) {
            Quick::do_quick_sort(arr, start, r);
        }
        if let Some(r) = pivot.checked_add(1) {
            Quick::do_quick_sort(arr, r, end);
        }
    }

    fn partition<T>(arr: &mut [T], start: usize, end: usize) -> usize
    where
        T: PartialOrd,
    {
        let mid = Quick::median(arr, start, start + (end - start + 1) / 2, end);
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
        T: PartialOrd,
    {
        if arr[idx1] < arr[idx2] {
            if arr[idx2] < arr[idx3] {
                return idx2;
            }
            if arr[idx1] < arr[idx3] {
                return idx3;
            }
            return idx1
        }
        if arr[idx1] < arr[idx3] {
            return idx1;
        }
        if arr[idx2] < arr[idx3] {
            return idx3;
        }
        idx2
    }
}

impl<T: PartialOrd> Sort<T> for Quick {
    /// 快速排序
    ///
    /// ```
    /// use algorithm::sort::{Quick, Sort};
    /// let mut arr = [3, 2, 1];
    /// Quick::sort(&mut arr);
    /// assert_eq!([1, 2, 3], arr);
    /// ```
    fn sort(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }

        Quick::do_quick_sort(arr, 0, arr.len() - 1);
    }
}

pub struct Shell;

impl<T: PartialOrd> Sort<T> for Shell {
    /// 希尔排序
    ///
    /// ```
    /// use algorithm::sort::{Shell, Sort};
    /// let mut arr = [3, 2, 1];
    /// Shell::sort(&mut arr);
    /// assert_eq!([1, 2, 3], arr)
    /// ```
    fn sort(arr: &mut [T]) {
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
}
