/// 插入排序
/// 
/// 稳定排序算法，平均时间复杂度为 O(n^2)，空间复杂度为 O(1)。
/// 
/// # Example
/// 
/// ```
/// use rust_demo::sort::insertion_sort;
/// let mut arr = [2, 1, 3];
/// insertion_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 3]);
/// ```
pub fn insertion_sort<T>(arr: &mut [T])
where
    T: Ord,
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

#[cfg(test)]
mod tests {
    use super::super::is_sort;
    use super::*;

    #[test]
    fn reverse_sorted() {
        let mut arr = vec![6, 5, 4, 3, 2, 1];
        insertion_sort(&mut arr);
        assert!(is_sort(&arr))
    }

    #[test]
    fn sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        insertion_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = vec![];
        insertion_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn one_element() {
        let mut arr = vec![1];
        insertion_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn repeat() {
        let mut arr = vec![2, 2, 1, 3, 3, 1];
        insertion_sort(&mut arr);
        assert!(is_sort(&arr));
    }
}
