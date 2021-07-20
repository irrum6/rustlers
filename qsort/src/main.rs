fn qsort(arr: &mut [i32], lo: isize, hi: isize) {
    if lo < hi {
        let pi = partition(arr, lo, hi);
        qsort(arr, lo, pi - 1);
        qsort(arr, pi + 1, hi);
    }
}

fn partition(arr: &mut [i32], lo: isize, hi: isize) -> isize {
    let pivot = arr[hi as usize];
    let mut i: isize = lo;

    for j in lo..hi + 1 {
        if arr[j as usize] < pivot {
            arr.swap(i as usize, j as usize);
            i += 1;
        }
    }
    // swap
    arr.swap(hi as usize, i as usize);
    return i;
}
#[test]
fn test() {
    let mut xes: [i32; 6] = [80, 10, 30, 90, 40, 50];
    qsort(&mut xes, 0, 5);
    assert_eq!(xes[5], 90);
    assert_eq!(xes[0], 10);
}
fn main() {
    let mut xes: [i32; 8] = [80,10,  30, 90, 40, 50, 20, 70];

    qsort(&mut xes, 0, 7);

    for x in xes.iter() {
        println!("x is {}", x);
    }

    println!("HellWorld");
}
