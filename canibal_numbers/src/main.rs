use std::env;
use std::fs;

mod utils;
use utils::dumb_shit as u;
use u::parse_u64 as parse_u64;
use u::slash_forward_find_duplicates as find_duplicates;
/**
 * Select smallest element to eat, which in reverse sorted 
 * vector would be in the end of it
 */
fn eat_from_back(v:&mut Vec<u64>,j:usize,len:usize,valcomp:u64){
    for m in (j..len).rev(){
        //dump_vec(&v);
        if v[j] >= valcomp{
            break;
        }
        if v[m]==0{
            continue;
        }
        if v[m]< v[j]{
           //consoome
           v[j] +=1;
           v[m] = 0;
       }              
   }
}
/**
 * for some cases is better to just move cursor forward
 * and eat next smalles number
 * this function does just that
 */
fn slash_forward_consoome(v:&mut Vec<u64>,j:usize,len:usize,valcomp:u64,self_count:u64){
    for k in j..len {
        if v[j] >= valcomp{
            break;
        }
        if v[k]==0{
            continue;
        }
        //may be skip selfN times?
        if v[k] == v[j] {
            continue;
        }                
        if v[k]< v[j]{                    
            //dump_vec(&v);
            //find duplicates for the current number and
            //next smaller number
            //if number of duplicates equal  or less than duplicates of 
            //next smaller number, then eat it
            let smallers = find_duplicates(&v, k, len, v[k]);
            if smallers ==1 || self_count > smallers{
                break;
            }
            //consoome 
             v[j] +=1;
             v[k] = 0;                     
        }  
    }
}
/**
 * Single query for single value
 * move over vector and find how many numbers can gain weight
 */
fn query(allnums:&mut Vec<u64>,valcmp:u64)->u64{
    let mut counter = 0;
       let mut v = allnums.clone();
       let len = v.len();
       
       for j in 0..len{
           if v[j] >= valcmp{
               continue;
           }
           if v[j] == 0 {
               continue;
           }           
            let self_n = find_duplicates(&v, j, len, v[j]);
            slash_forward_consoome(&mut v, j, len,  valcmp, self_n);
            eat_from_back(&mut v,j,len,valcmp);
       }
       for w in v {
           if w>=valcmp{
               counter += 1;
           }
       }
    return counter;
}
#[test]
fn test_query() {
    let mut data:Vec<u64> = vec![21, 9, 5, 8, 10, 1 ,3];
    data.sort();
    data.reverse();
    let valcmp:u64 = 10;
    let counter = query(&mut data,valcmp);
    assert_eq!(4,counter);
}

fn main() {
    let args:Vec<String> = env::args().collect();
    let filename = args[1].clone();

    //not a smart thing but will work for this small prog
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let params:Vec<&str> = contents.split_whitespace().collect();

    let mut n:u64 = 0;
    let mut q:u64 = 0;
    let mut nums:Vec<u64> = Vec::new();
    let mut all_numbers:Vec<u64> = Vec::new();
    let mut i = 0;
    
    for p in params {
        if i==0 {
            n = parse_u64(p);
        }
        if i==1 {
            q = parse_u64(p);
        }
        if n!=0 && i >1 && i <n+2{
            all_numbers.push(parse_u64(p));
        }
        if q!=0 && i >n+1 && i< n+q+2{
            nums.push(parse_u64(p));
        }
        i +=1;
    }

    all_numbers.sort();
    all_numbers.reverse();

    for i in 0..q{
       let counter = query(&mut all_numbers,nums[i as usize]);
       print!("{} ",counter);
    }
}