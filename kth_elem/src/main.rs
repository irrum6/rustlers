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

fn main() {
    use std::io::stdin;
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let num: usize = line.trim().parse().expect("usize");
    line.truncate(0);
    stdin().read_line(&mut line).unwrap();
    let strs: Vec<&str> = line.trim().split(" ").collect();
    let mut numberz: Vec<i32> = Vec::new();
    for s in strs {
        let v: i32 = s.parse().expect("i32");
        numberz.push(v);
    }

    if numberz.len() != num {
        panic!("too much arguments");
    }

    qsort(&mut numberz, 0, (num-1) as isize);
    line.truncate(0);
    stdin().read_line(&mut line).unwrap();
    let k: usize = line.trim().parse().expect("usize");

    println!("Hello,{}th biggest number is {}", k, numberz[num - 1 - k]);
}
