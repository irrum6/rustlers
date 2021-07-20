fn qsort(arr: &mut [i32], lo: isize, hi: isize) {
    if lo < hi {
        let pi = partition(arr, lo, hi);
        qsort(arr, lo, pi);
        qsort(arr, pi + 1, hi);
    }
}

fn partition(arr: &mut [i32], lo: isize, hi: isize) -> isize {
    if lo == hi {
        return lo;
    }
    if hi == lo + 1 {
        // compare
        if arr[hi as usize] < arr[lo as usize] {
            arr.swap(hi as usize, lo as usize);
        }
        return lo;
    }

    let pivot = arr[(lo + (hi - lo) / 2) as usize];

    let mut i: isize = lo - 1;
    let mut j: isize = hi + 1;

    loop {
        loop {
            i += 1;
            if arr[i as usize] >= pivot {
                break;
            }
        }
        loop {
            j -= 1;
            if arr[j as usize] <= pivot {
                break;
            }
        }
        if i >= j {
            return j;
        }
        arr.swap(i as usize, j as usize);
    }
}

#[test]
fn test() {
    let mut xes: [i32; 6] = [60, 10, 30, 20, 40, 50];
    let results: [i32; 6] = [10, 20, 30, 40, 50, 60];
    qsort(&mut xes, 0, 5);
    for i in 0..6 {
        println!("{} {} ", i, xes[i]);
        assert_eq!(xes[i], results[i]);
    }
}
#[test]
fn test8() {
    let mut xes: [i32; 8] = [80, 30, 90, 40, 50, 20, 10, 70];
    let results: [i32; 8] = [10, 20, 30, 40, 50, 70, 80, 90];
    qsort(&mut xes, 0, 7);
    for i in 0..8 {
        println!("{} {} ", i, xes[i]);
        assert_eq!(xes[i], results[i]);
    }
}
#[test]
#[ignore]
fn thousands() {
    use std::time::Instant;
    use std::time::{SystemTime, UNIX_EPOCH};

    use std::thread;
    let builder = thread::Builder::new()
        .name("bigstack".into())
        .stack_size(32 * 1024 * 1024);

    let handler = builder
        .spawn(|| {
            const N: usize = 1000000;
            let mut millia = [0; N];

            for i in (0..=N - 1).rev() {
                millia[i] = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .subsec_micros() as i32;
            }
            let before = Instant::now();

            qsort(&mut millia, 0, (N - 1) as isize);
            println!("Elapsed time: {:.2?}", before.elapsed());
        })
        .unwrap();

    handler.join().unwrap();
}
fn main() {
    let mut arr = [10, 200, 3, 400000, 5000, 60, 7, 800, 90];
    qsort(&mut arr, 0, 8);
    qsort(&mut arr, 0, 8);
    for e in arr.iter() {
        print!("{}, ", e);
    }
    println!("Hello");
}
