/*
* variant of trial division
* basic bad implementation
*/
fn td_bb(n:u64)->u64{
    let now = std::time::Instant::now();

    let mut primes:Vec<u64> = vec![2,3,5,7,11,13,17,19];

    let inc = [4, 2, 4, 2, 4, 6, 2, 6];

    let mut i = 23;
    let mut inc_index = 5;
    
    loop {
        let mut p = true;
        for v in &primes {
            if v*v > i {
                break;
            }
            if i%v == 0{
                p = false;
                break;
            }
        }
        if p {
            primes.push(i);
        }        
        i += inc[inc_index];
        inc_index += 1;
        if inc_index > inc.len()-1 {
            inc_index = 0;
        }
        if i > n {
            break;
        }
    }
    let elapsed = now.elapsed();
    println!("td_bb Elapsed: {:.3?}", elapsed);
    println!("found {} primes",primes.len());
    //return the last prime found
    return primes[primes.len()-1];
}
fn find_nth_prime(n:u64)->u64{
    
    let mut primes:Vec<u64> = vec![2,3,5,7,11,13,17,19];
    let mut primes_found = 8;

    if n < primes_found {
        return primes[n as usize];
    }
    
    let inc = [4, 2, 4, 2, 4, 6, 2, 6];

    let mut i = 23;
    let mut inc_index = 5;

    loop {
        let mut p = true;
        for v in &primes {
            if v*v > i {
                break;
            }
            if i%v == 0{
                p = false;
                break;
            }
        }
        if p {
            primes.push(i);
            primes_found += 1;            
        }
        if primes_found > n-1{
            return i;
        }        
        i += inc[inc_index];
        inc_index += 1;
        if inc_index > inc.len()-1 {
            inc_index = 0;
        }
    }
}

#[test]
fn test_td() {
    let v = td_bb(1000);
    assert_eq!(v,997);
}
#[test]
fn test_find_nth() {
    let v = find_nth_prime(1000);
    assert_eq!(v,7919);
}
fn main() {
   use std::env;
   let args:Vec<String> = env::args().collect();
   if args.len() <3 {
       println!("pass more parameters");
       return;
   }
   let num:u64 = args[2].trim().parse().expect("type a whole number");
   if args[1]=="count" ||args[1]=="N"{
       td_bb(num);
       return;
   }
   if args[1] =="nth" || args[1]=="find"{
       let p = find_nth_prime(num);
       println!("N:{} prime is {}",num,p);
       return;
   }
}