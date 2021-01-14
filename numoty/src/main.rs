
fn day_of_year(m:u8,d:u8,l:u8)->u16{
    if m==0 || d==0 {return 0;}
    let days_by_month:[u16;12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let mut result:u16 = days_by_month[(m-1)as usize] + (d as u16);

    if l>0 && m>2 {
        result += 1;
    }
    return result;
}
#[test]
fn test2020(){
    assert_eq!(day_of_year(1,1,1),1);
    assert_eq!(day_of_year(12, 31, 1),366);
}
#[test]
fn test2021() {
    assert_eq!(day_of_year(1,1,0),1);
    assert_eq!(day_of_year(12, 31, 0),365);
}

fn main() {
    use std::env;
    let args:Vec<String> = env::args().collect();
    if args.len() <3 {
        println!("missing parameters");
        return;
    }
    let month:u8 = args[1].trim().parse().expect("type a integer");
    let day:u8 = args[2].trim().parse().expect("type a integer");

    println!("{} {} is {}th day of year", month,day,day_of_year(month, day,0));
}
