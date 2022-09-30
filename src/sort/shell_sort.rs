/// 希尔排序
/// 
/// 不稳定排序算法，平均时间复杂度为 O(nlogn)，空间复杂度为 O(1)。
/// 
/// # Example
/// 
/// ```
/// use rust_demo::sort::shell_sort;
/// let mut arr = [2, 1, 3];
/// shell_sort(&mut arr);
/// assert_eq!(arr, [1, 2, 3]);
/// ```
pub fn shell_sort<T>(arr: &mut [T])
where
    T: Ord,
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

#[cfg(test)]
mod tests {
    use super::super::is_sort;
    use super::*;

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = vec![];
        shell_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn reverse_sorted() {
        let mut arr = vec![6, 5, 4, 3, 2, 1];
        shell_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        shell_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn one_element() {
        let mut arr = vec![1];
        shell_sort(&mut arr);
        assert!(is_sort(&arr));
    }

    #[test]
    fn repeat() {
        let mut arr = vec![2, 2, 1, 3, 3, 1];
        shell_sort(&mut arr);
        assert!(is_sort(&arr));
    }
}
