use std::env;

//euclidean gcd
fn legcd(mut a:u32,mut b:u32)->u32{
    while a != b {
        if a > b{
            a = a - b ;
        }            
        else{
            b = b - a ;
        }    
    }        
    return a;
}

fn is_even(a:u32)->bool{
    if a%2==0 {
        return true;
    }
    return false;
}
// Binary_GCD_algorithm
fn bgcd(a:u32, b :u32)->u32{
    // https://iq.opengenus.org/binary-gcd-algorithm/ 
    // AKA Stein’s algorithm
    if a == b { return a }
    if a == 0 { return b }
    if b == 0 { return a } 

    if is_even(a) && is_even(b){
        return bgcd(a >> 1, b >> 1) << 1;
    }
    if is_even(a){
        return bgcd(a >> 1, b);
    }
    if is_even(b){
        return bgcd(a, b >> 1);
    }    
    if a >= b{
        return bgcd((a-b) >> 1, b);
    }
    return bgcd(a, (b-a) >> 1);
}
// iterative binary gcd
fn bgcd_it(mut a:u32,mut b:u32)->u32{
    use std::cmp::min;
    use std::mem::swap;
    if a == 0 { return b }
    if b == 0 { return a }

    let i:u32= a.trailing_zeros();
    let j:u32= b.trailing_zeros();
    let k:u32= min(i,j);

    a >>= i;b >>= j;

    // both odd
    loop{
        if a > b {
            swap(&mut a, &mut b);
        }
        b= b - a;
        if b == 0 {
            break;
        }
        b >>= b.trailing_zeros();
    }
    return a<<k;
}

#[cfg(test)]
mod tests {
    use crate::bgcd;
    use crate::legcd;
    use crate::bgcd_it;
    #[test]
    fn test_bgcd () {
        let primes:[u32;10] =  [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        for p in primes.iter(){
            for q in primes.iter(){
                if p==q {
                continue;
                }
                let n1 = p * q;
                let n2 = q * q;
                let result = bgcd(n1,n2);
                assert_eq!(&result,q);                
            }
        }
    }
    #[test]
    fn test_legcd () {
        let primes:[u32;10] =  [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        for p in primes.iter(){
            for q in primes.iter(){
                if p==q {
                continue;
                }
                let n1 = p * q;
                let n2 = q * q;
                let result = legcd(n1,n2);
                assert_eq!(&result,q);                
            }
        }
    }
    #[test]
    fn test_bgcd_it () {
        let primes:[u32;10] =  [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        for p in primes.iter(){
            for q in primes.iter(){
                if p==q {
                continue;
                }
                let n1 = p * q;
                let n2 = q * q;
                let result = bgcd_it(n1,n2);
                assert_eq!(&result,q);                
            }
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("pass enough parameters to calculate");
        return;
    }
    
    let fst:u32 = args[1].trim().parse().expect("type a number");
    let snd:u32 = args[2].trim().parse().expect("type a number");

    println!("Greatest common divisor of {} and {} divisor is {}",fst,snd,bgcd(fst,snd));
    
    if args.len() > 3 {
        let opt = args[3].clone();
        
        if "le" == opt {
            println!("Greatest common divisor of {} and {} divisor is {}",fst,snd,legcd(fst,snd));
        }
        if "bin" == opt {
            println!("Greatest common divisor of {} and {} divisor is {}",fst,snd,bgcd(fst,snd));
        }
        if "bit" == opt {
            println!("Greatest common divisor of {} and {} divisor is {}",fst,snd,bgcd_it(fst,snd));
        }
        if "all" == opt {
            println!("Greatest common divisor of {} and {} divisor is {}",fst,snd,legcd(fst,snd));
            println!("Greatest common divisor of {} and {} divisor is {}",fst,snd,bgcd(fst,snd));
            println!("Greatest common divisor of {} and {} divisor is {}",fst,snd,bgcd_it(fst,snd));
        }
    }
}
