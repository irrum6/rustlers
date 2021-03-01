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
fn main() {
    println!("Hello, world!");
    wheel_buzz1(100);
}
