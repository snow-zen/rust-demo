mod bubble_sort_tests {
    use super::super::{Bubble, Sort};

    #[test]
    fn test_sort() {
        let mut arr = [3, 2, 1];
        Bubble::sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_sorted() {
        let mut arr = [1, 2, 3];
        Bubble::sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_repeat() {
        let mut arr = [2, 1, 1, 3];
        Bubble::sort(&mut arr);
        assert_eq!([1, 1, 2, 3], arr);
    }

    #[test]
    fn test_empty() {
        let mut arr: [i32; 0] = [];
        Bubble::sort(&mut arr);
        assert_eq!(0, arr.len());
    }
}

mod select_sort_tests {
    use super::super::{Select, Sort};

    #[test]
    fn test_sort() {
        let mut arr = [3, 2, 1];
        Select::sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_sorted() {
        let mut arr = [1, 2, 3];
        Select::sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_repeat() {
        let mut arr = [2, 1, 1, 3];
        Select::sort(&mut arr);
        assert_eq!([1, 1, 2, 3], arr);
    }

    #[test]
    fn test_empty() {
        let mut arr: [i32; 0] = [];
        Select::sort(&mut arr);
        assert_eq!(0, arr.len());
    }
}

mod insert_sort_tests {
    use super::super::{Insert, Sort};

    #[test]
    fn test_sort() {
        let mut arr = [3, 2, 1];
        Insert::sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_sorted() {
        let mut arr = [1, 2, 3];
        Insert::sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_repeat() {
        let mut arr = [2, 1, 1, 3];
        Insert::sort(&mut arr);
        assert_eq!([1, 1, 2, 3], arr);
    }

    #[test]
    fn test_empty() {
        let mut arr: [i32; 0] = [];
        Insert::sort(&mut arr);
        assert_eq!(0, arr.len());
    }
}

mod td_merge_sort_tests {
    use super::super::{Sort, TdMerge};

    #[test]
    fn test_sort() {
        let mut arr = [8, 7, 6, 5, 4, 3, 2, 1];
        TdMerge::sort(&mut arr);
        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8], arr);
    }

    #[test]
    fn test_sorted() {
        let mut arr = [1, 2, 3];
        TdMerge::sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_repeat() {
        let mut arr = [2, 1, 1, 3];
        TdMerge::sort(&mut arr);
        assert_eq!([1, 1, 2, 3], arr);
    }

    #[test]
    fn test_empty() {
        let mut arr: [i32; 0] = [];
        TdMerge::sort(&mut arr);
        assert_eq!(0, arr.len());
    }
}

mod bu_merge_sort_tests {
    use super::super::{BuMerge, Sort};

    #[test]
    fn test_sort() {
        let mut arr = [3, 2, 1];
        BuMerge::sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_sorted() {
        let mut arr = [1, 2, 3];
        BuMerge::sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_repeat() {
        let mut arr = [2, 1, 1, 3];
        BuMerge::sort(&mut arr);
        assert_eq!([1, 1, 2, 3], arr);
    }

    #[test]
    fn test_empty() {
        let mut arr: [i32; 0] = [];
        BuMerge::sort(&mut arr);
        assert_eq!(0, arr.len());
    }
}

mod quick_sort_tests {
    use super::super::{Quick, Sort};

    #[test]
    fn test_sort() {
        let mut arr = [9, 8, 7, 6, 5, 4, 3, 2, 1];
        Quick::sort(&mut arr);
        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8, 9], arr);
    }

    #[test]
    fn test_sorted() {
        let mut arr = [1, 2, 3];
        Quick::sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_repeat() {
        let mut arr = [2, 1, 1, 3];
        Quick::sort(&mut arr);
        assert_eq!([1, 1, 2, 3], arr);
    }

    #[test]
    fn test_empty() {
        let mut arr: [i32; 0] = [];
        Quick::sort(&mut arr);
        assert_eq!(0, arr.len());
    }
}

mod shell_sort_tests {
    use super::super::{Shell, Sort};

    #[test]
    fn test_sort() {
        let mut arr = [3, 2, 1];
        Shell::sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_sorted() {
        let mut arr = [1, 2, 3];
        Shell::sort(&mut arr);
        assert_eq!([1, 2, 3], arr);
    }

    #[test]
    fn test_repeat() {
        let mut arr = [2, 1, 1, 3];
        Shell::sort(&mut arr);
        assert_eq!([1, 1, 2, 3], arr);
    }

    #[test]
    fn test_empty() {
        let mut arr: [i32; 0] = [];
        Shell::sort(&mut arr);
        assert_eq!(0, arr.len());
    }
}
