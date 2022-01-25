// we don't need to consume values there
fn find_first_and_last_positions(arr: &[i32], search: i32) -> (isize, isize) {
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
#[test]
fn test1() {
    let arr = [0, 1, 2, 3, 3, 3, 3, 4, 4, 5, 6];
    let result1 = find_first_and_last_positions(&arr, 3);
    assert_eq!(result1.0, 3);
    let result1 = find_first_and_last_positions(&arr, 4);
    assert_eq!(result1.0, 7);
    let result1 = find_first_and_last_positions(&arr, 5);
    assert_eq!(result1.0, 9);
    let result1 = find_first_and_last_positions(&arr, 6);
    assert_eq!(result1.0, 10);
}

fn main() {
    let arr = [0, 1, 2, 3, 3, 3, 3, 4, 4, 5, 6];
    println!("{:?}", find_first_and_last_positions(&arr, 3));
}
