//iterative quick sort
fn iqsort(arr:&mut[i32], mut lo:usize, mut hi:usize){
    let mut stack:Vec<usize> = Vec::new();

    stack.push(lo);
    stack.push(hi);
    
    loop{
        hi = stack.pop().unwrap();
        lo = stack.pop().unwrap();

        let pi = partition(arr, lo, hi);
        if  pi - 1 > lo { 
            stack.push(lo);
            stack.push(pi-1);
        }
        if pi + 1 < hi { 
            stack.push(pi+1);
            stack.push(hi);
        }  
        if stack.len()==0{
            break;
        }
    }
}

fn partition(arr:&mut[i32],lo:usize,hi:usize)->usize{
    let pivot = arr[hi];
    
    let mut i = lo - 1;

    for j in lo..hi{
        if arr[j] < pivot {
            i += 1;
            if i==j {
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
    iqsort(&mut xes,1,5);
    assert_eq!(xes[5],90);
}
fn main() {
    let mut xes:[i32;8] = [10, 80, 30, 90, 40, 50, 20, 70];

    iqsort(&mut xes,1,7);

    for x in xes.iter() {
        println!("x is {}", x);
    }
    println!("Hello, World");
}
