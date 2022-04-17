fn fibonacci(n: usize) -> i64 {
    if n == 1 || n == 2 {
        return 1;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn cached_fibonacci(n: usize) -> i64 {
    let x = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    if n < 12 {
        return x[n - 1];
    }
    return cached_fibonacci(n - 1) + cached_fibonacci(n - 2);
}

fn iter_fibonacci(n: usize) -> i64 {
    if n == 1 || n == 2 {
        return 1;
    }
    let mut a = 1;
    let mut b = 1;

    for _i in 3..=n {
        a = a + b;
        b = a - b;
    }
    return a;
}

fn cater_fibonacci(n: usize) -> i64 {
    let x = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    if n < 12 {
        return x[n - 1];
    }
    let mut a = x[11];
    let mut b = x[10];

    for _i in 13..=n {
        a = a + b;
        b = a - b;
    }
    return a;
}

#[test]
fn test() {
    //first 20 fibonaci numbers;
    //0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 4181 6765
    assert_eq!(fibonacci(1), 1);
    assert_eq!(cached_fibonacci(2), 1);
    assert_eq!(cached_fibonacci(3), 2);
    assert_eq!(cached_fibonacci(4), 3);
    assert_eq!(cached_fibonacci(5), 5);
    assert_eq!(cached_fibonacci(6), 8);
    assert_eq!(cached_fibonacci(7), 13);
    assert_eq!(cached_fibonacci(8), 21);
    assert_eq!(cater_fibonacci(9), 34);
    assert_eq!(cached_fibonacci(10), 55);

    assert_eq!(cached_fibonacci(16), 987);
    assert_eq!(iter_fibonacci(20), 6765);
}

fn main() {
    use std::io::stdin;
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let k: usize = line.trim().parse().expect("usize");

    let fib = iter_fibonacci(k);
    println!("Hello,{}th fibonacci number is {}", k, fib);
}
