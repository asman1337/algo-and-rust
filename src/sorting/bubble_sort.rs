pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let mut n = arr.len();
    while n > 0 {
        let mut last_modified = 0;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                last_modified = i;
            }
        }
        n = last_modified;
    }
}