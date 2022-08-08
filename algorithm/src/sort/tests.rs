mod bubble_sort_tests {
    use super::super::bubble_sort;

    #[test]
    fn test_sort() {
        let mut arr = [3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_sorted() {
        let mut arr = [1, 2, 3];
        bubble_sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_repeat() {
        let mut arr = [2, 1, 1, 3];
        bubble_sort(&mut arr);
        assert_eq!([1, 1, 2, 3], arr);
    }

    #[test]
    fn test_empty() {
        let mut arr: [i32; 0] = [];
        bubble_sort(&mut arr);
        assert_eq!(0, arr.len());
    }
}

mod select_sort_tests {
    use super::super::select_sort;

    #[test]
    fn test_sort() {
        let mut arr = [3, 2, 1];
        select_sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_sorted() {
        let mut arr = [1, 2, 3];
        select_sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_repeat() {
        let mut arr = [2, 1, 1, 3];
        select_sort(&mut arr);
        assert_eq!([1, 1, 2, 3], arr);
    }

    #[test]
    fn test_empty() {
        let mut arr: [i32; 0] = [];
        select_sort(&mut arr);
        assert_eq!(0, arr.len());
    }
}