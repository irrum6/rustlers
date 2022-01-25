pub mod finder {
    // we don't need to consume values there
    pub fn find_first_and_last_positions(arr: &[i32], search: i32) -> (isize, isize) {
        // In a sorted array
        let mut start = -1;
        let mut last = -1;
        let mut count = -1;
        for e in arr {
            count += 1;
            if e == &search {
                if start == -1 {
                    start = count;
                }
                last = count;
            }
        }
        return (start, last);
    }
    fn thrash_bin_search_first(arr: &[i32], search_item: i32) -> usize {
        if arr[0] == search_item {
            return 0;
        }
        let mut left = 0;
        let mut right = arr.len() - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if arr[mid] == search_item && arr[mid - 1] < search_item {
                return mid;
            } else if arr[mid] < search_item {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        return arr.len();
    }
    fn thrash_bin_search_last(arr: &[i32], search_item: i32) -> usize {
        let lengo = arr.len();
        if arr[lengo - 1] == search_item {
            return lengo - 1;
        }
        let mut left = 0;
        let mut right = arr.len() - 1;
        while left <= right {
            let mid = (left + right) / 2;
            if arr[mid] == search_item && arr[mid + 1] > search_item {
                return mid;
            } else if arr[mid] > search_item {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        return arr.len();
    }
    pub fn first_and_last_positions_bin_search(arr: &[i32], search: i32) -> (usize, usize) {
        let n = arr.len();
        let first = thrash_bin_search_first(arr, search);
        let last = thrash_bin_search_last(arr, search);
        if first == n {
            return (n, n);
        }
        return (first, last);
    }
}
