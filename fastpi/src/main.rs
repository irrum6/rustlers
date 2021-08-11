//adapted from
//http://stanleyrabinowitz.com/bibliography/spigot.pdf

use std::time::Instant;
fn main() {
    const n: u64 = 100000;
    const len: u64 = (10 * n) / 3;

    let mut a: [u64; len as usize] = [2; len as usize];
    let mut nines: u64 = 0;
    let mut predigit: u64 = 0;

    let mut x: u64 = 0;
    let mut q: u64 = 0; //quotient

    let before = Instant::now();
    for j in 1..=n {
        q = 0;
        for i in (1..len).rev() {
            x = 10 * a[i as usize] + q * i as u64;
            a[i as usize] = x % (2 * i as u64 - 1);
            q = x / (2 * i as u64 - 1);
        }
        a[1] = q % 10;
        q = q / 10;
        if q == 9 {
            nines = nines + 1;
        } else if q == 10 {
            print!("{}", predigit + 1);
            for k in 1..=nines {
                print!("0");
            }
            predigit = 0;
            nines = 0;
        } else {
            print!("{}", predigit);
            predigit = q;
            if nines != 0 {
                for k in 1..=nines {
                    print!("9");
                }
                nines = 0;
            }
        }
    }
    println!("{}", predigit);
    println!("Elapsed time: {:.3?}", before.elapsed());
}
