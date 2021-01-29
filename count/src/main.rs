// in sorted array find occurences
fn linear_count(arr:&[u32],x:u32)->u32{
    let len = arr.len();
    let mut i = 0;
    let mut count = 0;
    loop{        
        if arr[i] == x {
            count += 1;
        }
        i += 1;
        if i > len-1 {
            break;
        }
    }
    return count;
}
// 
fn bsearch(arr:&[u32],l:usize,r:usize,x:u32)->(usize,i8){

    if r < l {
        return (r,-1);
    }       
  
    let mid:usize = l + (r - l) / 2; 
  
    if arr[mid] == x {
        return (mid,0);
    }

    if arr[mid] > x {
        let zero = bsearch(arr, l, mid - 1, x).0;
        return (zero,0);
    } 

    let zero = bsearch(arr, mid + 1, r, x).0;
    return (zero,0);
}

fn count_elements(arr:&[u32],n:usize,x:u32)->u32{
    let searched = bsearch(arr, 0, n-1, x);

    if searched.1==-1 {
        return 0;
    }
    let mut count = 1;
    // special cases
    if searched.0==0{
        return 1;
    }
    let mut left = searched.0 -1;

    while arr[left] == x{        
        count += 1;
        if left==0 {
            break
        }
        left -= 1;
    }
    let mut right = searched.0+1;
    while right < n && arr[right] == x{
        count += 1;
        right += 1;
    }
    return count;
}
// 
fn bsearch2(arr:&[u32],n:usize,x:u32)->usize{
    let mut right = n-1;
    let mut left = 0;
    let mut middle; 
    loop{
        if right < left {
            // arrays can't be indexed with int
            //so we return number greater than size as indicator of not founding with bsearch there
            //check when on return
            return n+1;
        }
        middle = left +(right - left)/2;
        if arr[middle] ==x {
            return middle;
        }
        if arr[middle] > x {
            //then search on left
            right = middle -1;
            continue;
        }
        left = middle +1;
    }
    //return middle;
}

fn count_elements2(arr:&[u32],n:usize,x:u32)->u32{
    let index = bsearch2(arr,n, x);
    // greater than size, not found there
    if index > n {
        return 0;
    }
    if index ==0 {
        return 1;
    }
    let mut count = 1;
    let mut left = index -1;
    while arr[left] == x{
        count += 1;
        if left==0 {
            break
        }
        left -= 1;
    }
    let mut right = index+1;
    while right < n && arr[right] == x{
        count += 1;
        right += 1;
    }
    return count;
}

#[test]
fn sixes() {
    let x = 6;
    let xx:[u32;6] = [1,3,4,5,6,6];
    assert_eq!(linear_count( &xx,x),2);
    assert_eq!(count_elements( &xx,6,x),2);
    assert_eq!(count_elements2( &xx,6,x),2);
}
#[test]
fn twos() {
    let x = 2;
    let xx:[u32;10] = [2,2,2,2,3,3,4,5,6,6];
    assert_eq!(linear_count( &xx,x),4);
    assert_eq!(count_elements( &xx,10,x),4);
    assert_eq!(count_elements2( &xx,10,x),4);
}
#[test]
fn threes() {
    let x = 3;
    let xx:[u32;8] = [3,4,4,5,6,6,8,9];
    assert_eq!(linear_count( &xx,x),1);
    assert_eq!(count_elements( &xx,10,x),1);
    assert_eq!(count_elements2( &xx,10,x),1);
}

fn main() {
    let xx:[u32;10] = [1,2,3,3,4,5,6,6,7,8];
    println!("{}",linear_count( &xx,1));
    println!("{}",count_elements( &xx,10,2));
    println!("{}",count_elements2( &xx,10,3));
    // println!("Hello, world!");
}
