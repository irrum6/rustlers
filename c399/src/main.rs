fn get_letter_value(c: char) -> u32 {
    return match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        _ => 0,
    };
}

fn get_letter_sum(s: String) -> u32 {
    if s.len() == 0 {
        return 0;
    }
    // convert to char array
    // map chars to integers
    // fold(reduce) integer array to single value
    return s
        .chars()
        .map(|e| get_letter_value(e))
        .fold(0, |acc, x| acc + x);
}

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("{}", get_letter_sum(args[1].clone()));
    }
}
