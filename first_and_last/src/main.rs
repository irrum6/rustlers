use first_and_last::finder::{
    find_first_and_last_positions as fist_finder, first_and_last_positions_bin_search as finder,
};

#[test]
fn test_fist_finder() {
    let arr = [0, 1, 2, 3, 3, 3, 3, 4, 4, 5, 6];
    let result = fist_finder(&arr, 3);
    assert_eq!(result.0, 3);
    let result = fist_finder(&arr, 4);
    assert_eq!(result.0, 7);
    let result = fist_finder(&arr, 5);
    assert_eq!(result.0, 9);
    let result = fist_finder(&arr, 6);
    assert_eq!(result.0, 10);
    let result = fist_finder(&arr, 8);
    assert_eq!(result.1, -1);
}
#[test]
fn test_second_function() {
    let arr = [0, 1, 2, 3, 3, 3, 3, 4, 4, 5, 6];
    let total_count = arr.len();

    let result = finder(&arr, 3);
    assert_eq!(result.0, 3);
    assert_eq!(result.1, 6);
    let result = finder(&arr, 4);
    assert_eq!(result.0, 7);
    let result = finder(&arr, 5);
    assert_eq!(result.0, 9);
    let result = finder(&arr, 6);
    assert_eq!(result.0, 10);
    assert_eq!(result.1, 10);
    //legit count is from  0 (zero) to n-1 (array size minus one)
    //function returns array size if not found
    //seems counterintuitive but is easier than wrapping isize
    let result = finder(&arr, 8);
    assert_eq!(result.1, total_count);
}

fn main() {
    let arr = [0, 1, 2, 3, 3, 3, 3, 4, 4, 5, 6];
    println!("{:?}", fist_finder(&arr, 3));
    println!("{:?}", finder(&arr, 4));
    println!("{:?}", finder(&arr, 6));
}
