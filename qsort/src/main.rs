fn qsort(arr:&mut[i32],lo:usize,hi:usize){
    if lo < hi {
        let pi = partition(arr, lo, hi);
        qsort(arr, lo ,pi-1);
        qsort(arr, pi+1 ,hi);
    }
}

fn partition(arr:&mut[i32],lo:usize,hi:usize)->usize{
    let pivot = arr[hi];
    
    let mut i = lo - 1;

    for j in lo..=(hi-1){
        if arr[j] < pivot {
            i += 1;
            if i==j {
                //if not this we could accidentally zero things with our inline swap
                continue;
            }
            // inline swap mem::swap complained
            arr[j] = arr[i] + arr[j];
            arr[i] = arr[j] - arr[i];
            arr[j] -= arr[i];
        }
    }
    // swap
    if i != hi-1{
        arr[hi] = arr[i+1] + arr[hi];
        arr[i+1] = arr[hi] - arr[i+1];
        arr[hi] -= arr[i+1];
    }  

    return i+1;
}
#[test]
fn test() {
    let mut xes:[i32;6] = [10, 80, 30, 90, 40,50];
    qsort(&mut xes,1,5);
    assert_eq!(xes[5],90);
}
fn main() {
    let mut xes:[i32;8] = [10, 80, 30, 90, 40, 50, 20, 70];

    qsort(&mut xes,1,7);

    for x in xes.iter() {
        println!("x is {}", x);
    }

    println!("HellWorld");  
}
