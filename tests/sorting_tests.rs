#[cfg(test)]
mod tests {
    use algo_and_rust::sorting::bubble_sort::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        let mut array = vec![64, 34, 25, 12, 22, 11, 90];
        bubble_sort(&mut array);
        assert_eq!(array, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_bubble_sort_empty() {
        let mut array: Vec<i32> = Vec::new();
        bubble_sort(&mut array);
        assert!(array.is_empty());
    }

    #[test]
    fn test_bubble_sort_single_element() {
        let mut array = vec![1];
        bubble_sort(&mut array);
        assert_eq!(array, vec![1]);
    }

    #[test]
    fn test_bubble_sort_already_sorted() {
        let mut array = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut array);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_reverse_order() {
        let mut array = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut array);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }
}
