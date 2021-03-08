fn wheel_buzz1 (num:u64)->u64{
    let wheel  = [3,2,1,3,1,2,3];
    let mut n =0;
    let mut i =0;
    loop{
        n += wheel [i];
        i += 1;
        if i > 6 {
            i = 0;
        }
        if n> num {
            break;
        }
        if n%15==0 {
            println!("FizzBuzz {}",n);
            continue;
        }
        if n%3==0 {
            println!("Fizz {}",n);
            continue;
        }
        if n%5==0 {
            println!("Buzz {}",n);
            continue;
        }
    }
    return n;
} 
mod moditer;
use moditer::it as it;
use it::FizzBuzzWheel as FizzBuzzWheel;
mod test;

fn main() {
    use std::env;
    let args:Vec<String> = env::args().collect();

    if args.len() < 2{
        wheel_buzz1(100);
        return;
    }
    println!("Gela var, gelava!");
    let num:u64 = args[1].trim().parse().expect("integer shall be");
    //wheel_buzz1(num);

    let w = FizzBuzzWheel::new(num);

    for e in w{
        if e%15==0 {
            println!("FizzBuzz {}",e);
            continue;
        }
        if e%3==0 {
            println!("Fizz {}",e);
            continue;
        }
        if e%5==0 {
            println!("Buzz {}",e);
            continue;
        }
    }
    
}
