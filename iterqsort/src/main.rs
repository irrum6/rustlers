//iterative quick sort
fn iqsort(arr: &mut [i32], mut lo: isize, mut hi: isize) {
    let mut stack: Vec<isize> = Vec::new();

    stack.push(lo);
    stack.push(hi);

    loop {
        hi = stack.pop().unwrap();
        lo = stack.pop().unwrap();

        let pi = partition(arr, lo, hi);
        if pi - 1 > lo {
            stack.push(lo);
            stack.push(pi - 1);
        }
        if pi + 1 < hi {
            stack.push(pi + 1);
            stack.push(hi);
        }
        if stack.len() == 0 {
            break;
        }
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
    let mut xes: [i32; 8] = [80, 30, 90, 40, 50, 20, 10, 70];
    iqsort(&mut xes, 0, 7);
    // compare sorted array to array in order
    let results: [i32; 8] = [10, 20, 30, 40, 50, 70, 80, 90];
    for i in 0..=7 {
        println!("{} {} ", i, xes[i]);
        assert_eq!(xes[i], results[i]);
    }
}
fn main() {
    let mut xes: [i32; 12] = [10, 80, 30, 90, 40, 50, 20, 70, 60, 120, 110, 100];

    iqsort(&mut xes, 1, 11);

    for x in xes.iter() {
        println!("x is {}", x);
    }
    println!("Hello, World");
}
