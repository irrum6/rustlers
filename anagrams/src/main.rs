// we don't need to consume values there
fn is_anagram(s1: &String, s2: &String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    //convert unicode codepoint value to unsigned 32bit intger and then sum it with fold
    let sum1 = s1.chars().map(|e| e as u32).fold(0, |acc, x| acc + x);
    let sum2 = s2.chars().map(|e| e as u32).fold(0, |acc, x| acc + x);
    //compare sums, for anagrams it should be equal
    sum1 == sum2
}

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("{}", is_anagram(&args[1], &args[2]));
    }
}
