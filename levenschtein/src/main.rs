fn min3(a:usize,b:usize,c:usize)->usize{
    let mut min = a;
    if b < min { min = b;}
    if c < min { min = c;}
    return min;
}

fn distance(s1:&str,s2:&str)->usize{
    use std::mem::swap;

    let str1:Vec<char> = s1.chars().collect();
    let str2:Vec<char> = s2.chars().collect();

    let m = str1.len();
    let n = str2.len();

    let mut v0:Vec<usize> = vec![0;n+1];
    let mut v1:Vec<usize> =  vec![0;n+1];
    
    for i in 0..=n {
        v0[i]=i;
    }

    for i in 0..=m-1 {
        for j in 0..=n-1 {
            let remove = v0[j+1]+1;
            let insert = v1[j]+1;
            let subs:usize;
            if str1[i]==str2[j] {
                subs = v0[j];
            }else {
                subs = v0[j]+1;
            }
            v1[j + 1] = min3(remove, insert, subs);

        }
        swap(&mut v0, &mut v1);
    }
    return v0[n];
}
#[test]
fn kitten(){    
    let result:usize = distance("kitten","sitting");
    assert_eq!(result,3);
}

fn main() {
    use std::env;
    
    let args:Vec<String> = env::args().collect();

    if args.len()>2 {       
        println!("distance is {}", distance(&args[1],&args[2]));
    }
        
}
