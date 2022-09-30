pub use self::bubble_sort::bubble_sort;
pub use self::heap_sort::heap_sort;
pub use self::insertion_sort::insertion_sort;
pub use self::merge_sort::{bu_merge_sort, td_merge_sort};
pub use self::quick_sort::quick_sort;
pub use self::selection_sort::selection_sort;
pub use self::shell_sort::shell_sort;

mod bubble_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod selection_sort;
mod shell_sort;

#[allow(dead_code)]
fn is_sort<T>(arr: &[T]) -> bool
where
    T: PartialOrd,
{
    if arr.is_empty() {
        return true;
    }

    let mut prev = &arr[0];
    for it in arr.iter().skip(1) {
        if prev > it {
            return false;
        }
        prev = it;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_sorted() {
        assert!(is_sort(&[] as &[isize]));
        assert!(is_sort(&["a"]));
        assert!(is_sort(&[1, 2, 3]));
        assert!(is_sort(&[0, 1, 1]));

        assert_eq!(is_sort(&[1, 0]), false);
        assert_eq!(is_sort(&[2, 3, 1, -1, 5]), false);
    }
}
