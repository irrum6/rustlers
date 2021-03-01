// a3 +b3  = c3 +d3
fn main() {
    use std::env;

    let args:Vec<String> = env::args().collect();
    if args.len() <2 {
        println!("Pass a parameter. number [1,1000)");
        return;
    }

    let n:u64 = args[1].trim().parse().expect("whole number needed");
    if n>1000 {
        println!("Value was greater than expected, ignored.");
        return;
    } 

    use std::collections::HashMap;

    let mut all_cubes:HashMap<u64,(u64,u64)> = HashMap::new();
    let mut good_cubes:HashMap<(u64,u64),(u64,u64)> = HashMap::new();

    for c in 1..n{
        for d in 1..n{
             if c==d {
                 continue;
             }
            let result = (c*c*c) + (d*d*d);
            let r = all_cubes.get(&result);
            if r.is_some(){
                let s = r.unwrap();
                //check for opposites
                if s.1==c || s.1==d || s.1==s.0 {
                    continue;
                }
                good_cubes.insert((c,d),(s.0,s.1));
            }
            all_cubes.insert(result, (c,d));

        }
    }

    for (k,v) in &good_cubes{
        //let r = good_cubes.get(&k).unwrap();
        println!("{}^3 + {}^3 = {}^3+{}^3",k.0,k.1,v.0,v.1);
    }
    
    println!("Found {:?}  numbers", good_cubes.len());
    println!("Hello, world!");
}
