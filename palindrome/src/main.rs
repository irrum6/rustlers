fn is_palindrome(s:&str)->bool{
    let chars:Vec<char> = s.to_lowercase().chars().collect();
    let len = chars.len();
    for i in 0..= (len-1)/2 {
        if chars[i] !=chars[len-1-i]{
            return false;
        }
    }
    return true;
}
#[test]
fn test () {
    let test_data:[&str;5] = ["abba","Guru","Abba","gelati","galag"];
    let results:[bool;5] = [true,false,true,false,true];

    for i in 0..=4{
        assert_eq!(is_palindrome(test_data[i]),results[i]);
    }
}
fn main() {
    use std::env;
    let xargs:Vec<String> = env::args().collect();

    if xargs.len() < 2 {
        println!("pass a parameter");
    }
    let filler= if is_palindrome(&xargs[1]){""}else{"not"};

    println!("{} is {} a palindrome", xargs[1],filler);
}