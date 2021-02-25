use std::env;
use std::fs;

fn parse_u64(s:&str)->u64{
    let wasnt = "Parameter wasn't a number, oughta be";
    return s.parse().expect(wasnt);
}
/**
 * Seek Forwad Find Duplicates
 */
fn slash_forward_find_duplicates(arr:&Vec<u64>,i:usize,len:usize,v:u64)->u64{
    let mut count = 0;
    for j in i..len{
        //println!("{}",arr[j]);
        if arr[j] != v {
            break;
        }
        count +=1;
    }
    return count;
}
fn dump_vec(v:&Vec<u64>){
    for vv in v{
        print!("{}-",vv);
    }
    print!("\n");
}

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

fn slash_forward_consoome(v:&mut Vec<u64>,j:usize,len:usize,valcomp:u64,self_count:u64){
    //it stops if ate to the value or all elements were tried
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
            let smallers = slash_forward_find_duplicates(&v, k, len, v[k]);
            if smallers ==1 || self_count > smallers{
                break;
            }
            //consoome 
             v[j] +=1;
             v[k] = 0;                     
        }  
    }
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
    let mut i=0;
    
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
       let mut counter = 0;
       let mut v = all_numbers.clone();
       let len = v.len();
       
       for j in 0..len{
           if v[j] >= nums[i as usize]{
               continue;
           }
           if v[j] == 0 {
               continue;
           }           
            let self_n = slash_forward_find_duplicates(&v, j, len, v[j]);
            slash_forward_consoome(&mut v, j, len,  nums[i as usize], self_n);
            eat_from_back(&mut v,j,len,nums[i as usize]);
       }
       for w in v {
           if w>=nums[i as usize]{
               counter += 1;
           }
       }
       print!("{} ",counter);
    }
}