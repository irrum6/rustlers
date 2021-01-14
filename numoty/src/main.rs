
fn day_of_year(m:u8,d:u8,l:bool)->u16{
    if m==0 || d==0 {return 0;}
    let days_by_month:[u16;12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let mut result:u16 = days_by_month[(m-1)as usize] + (d as u16);

    if l && m>2 {
        result += 1;
    }
    return result;
}

fn doty(date:&str,sep:char,reverse:bool)->u16{
    let dates:Vec<&str> = date.split(sep).collect();
    //using sane pattern here day-month-year
    //and its reversal year-month-day
    let year:u16 = dates[2].trim().parse().expect("must be a number");
    let mut leap:bool=false;
    if year % 4 == 0 && year != 2100  {    
        leap = true;    
    }
    let month= strmon_to_num(dates[1]);
    let day = strday_to_num(dates[0]);
    return day_of_year(month, day, leap);
}
fn strday_to_num(d:&str)->u8{
    let result = match d {
        "1st" => 1,
        "2nd" => 2,
        "3rd" => 3,
        "31st" => 31,
        _ => d.replace("th","").trim().parse().expect("must be a number less than 31")
    };
    return result;
}
fn strmon_to_num(m:&str)->u8 {
    let result = match m {
        "January"|"Jan" => 1,
        "February"|"Feb" => 2,
        "March"|"Mar" => 3,
        "April"|"Apr" => 4,
        "May"=>5,
        "June"|"Jun"=>6,
        "July"|"Jul"=>7,
        "August"|"Aug"=>8,
        "September"|"Sep"=>9,
        "October"|"Oct"=>10,
        "November"|"Nov"=>11,
        "December"|"Dec"=>12,
        _ => 0
    };
    return result;
}

#[test]
fn test2020(){
    assert_eq!(day_of_year(1,1,true),1);
    assert_eq!(day_of_year(12, 31, true),366);
}
#[test]
fn test2021() {
    assert_eq!(day_of_year(1,1,false),1);
    assert_eq!(day_of_year(12, 31, false),365);
}
#[test]
fn test20211(){
    assert_eq!(doty("06-January-2021",'-',false),6);
    assert_eq!(doty("31-December-2021",'-',false),365);
}

fn main() {
    use std::env;
    let args:Vec<String> = env::args().collect();
    if args.len() <3 {
        println!("missing parameters");
        return;
    }
    if args[1]=="v"{
        println!("{} - day of the year:{}",args[2],doty(args[2].as_ref(), '-', false));
        return;
    }
    let month:u8 = args[1].trim().parse().expect("type a integer");
    let day:u8 = args[2].trim().parse().expect("type a integer");

    println!("{} {} is {}th day of year", month,day,day_of_year(month, day,false));
}
