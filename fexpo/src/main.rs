fn fast_power (_base:i64,pow:u64)->i64{
    let mut result = 1;
    let mut power = pow;
    let mut base = _base;
    loop{
        if power < 1 {
            break;
        }
        if power %2==0 {
            power = power /2;
            base = base * base;
            continue;
        }
        power -= 1;
        result = result * base;

        power = power /2;
        base = base * base;
    }
    return result;
}
fn fast_power_mod (_base:i64,pow:u64)->i64{
    const MOD:i64 = 1000000007;
    let mut result = 1;
    let mut power = pow;
    let mut base = _base;
    loop{
        if power < 1 {
            break;
        }
        if power %2==1 {
            power -= 1;
            result = (result * base)%MOD;
        }       

        power = power /2;
        base = (base * base)%MOD;
    }
    return result;
}
fn fpower (_base:i64,pow:u64)->i64{
    let mut result = 1;
    let mut power = pow;
    let mut base = _base;
    loop{
        if power < 1 {
            break;
        }
        if power %2==1 {
            power -= 1;
            result = result * base;
        }       

        power = power /2;
        base = base * base;
    }
    return result;
}
#[test]
fn simple(){
    assert_eq!(fast_power(2,9),512);
    assert_eq!(fpower(2,9),512);
}
#[test]
fn squares1to20 () {
    let test_data:[i64;9] = [1,2,3,4,5,6,7,8,9];
    let results:[i64;9] = [1,4,9,16,25,36,49,64,81];
    for i in 0..=8{
       assert_eq!(fast_power(test_data[i],2),results[i]);
    }
    let test_data:[i64;10] = [10,11,12,13,14,15,16,17,18,19];
    let results:[i64;10] = [100,121,144,169,196,225,256,289,324,361];
    for i in 0..=9 {
       assert_eq!(fast_power(test_data[i],2),results[i]);
    }    
}
#[test]
fn squares20to50 (){
    let test_data:[i64;10] = [20,21,22,23,24,25,26,27,28,29];
    let results:[i64;10] = [400,441,484,529,576,625,676,729,784,841];
    for i in 0..=9 {
       assert_eq!(fpower(test_data[i],2),results[i]); 
    }
    let test_data:[i64;10] = [30,31,32,33,34,35,36,37,38,39];
    let results:[i64;10] = [900,961,1024,1089,1156,1225,1296,1369,1444,1521];
    for i in 0..=9 {
       assert_eq!(fpower(test_data[i],2),results[i]); 
    }
    let test_data:[i64;10] = [40,41,42,43,44,45,46,47,48,49];
    let results:[i64;10] = [1600,1681,1764,1849,1936,2025,2116,2209,2304,2401];
    for i in 0..=9 {
       assert_eq!(fpower(test_data[i],2),results[i]); 
    }
}
#[test]
fn threes () {
    let test_data:[i64;11] = [3,13,23,33,43,53,63,73,83,93,103];
    let results:[i64;11] = [9,169,529,1089,1849,2809,3969,5329,6889,8649,10609];
    for i in 0..=9 {
       assert_eq!(fast_power_mod(test_data[i],2),results[i]); 
    }
}
#[test]
fn twos () {
    let base = 2;
    let results:[i64;17] = [1,2,4,8,16,32,64,128,256,512,1024,2048,4096,8192,16384,32768,65536];
    for i in 1..=16 {
        assert_eq!(fast_power(base,i as u64),results[i]);
    }
}
#[test]
fn third(){
    let test_data:[i64;9] = [1,2,3,4,5,6,7,8,9];
    let results:[i64;9] = [1,8,27,64,125,216,343,512,729];
    for i in 0..=8{
       assert_eq!(fast_power(test_data[i],3),results[i]); 
    }
}

fn main() {
    use std::env;
    let args:Vec<String> = env::args().collect();
    if args.len() <3 {
        println!("missing parameters");
        return;
    }
    let base:i64 = args[1].trim().parse().expect("type a integer");
    let power:u64 = args[2].trim().parse().expect("type a integer");

    println!("{} at power {} is {}", base,power,fast_power(base, power));
}
