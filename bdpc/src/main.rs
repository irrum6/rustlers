use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

use std::collections::HashMap;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut map: HashMap<u16, String> = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(linevar) = line {
                let mut split = linevar.split(",");

                let m: u16 = split.next().unwrap().trim().parse().expect("NaN");
                let d: u16 = split.next().unwrap().trim().parse().expect("NaN");
                let name = split.next().unwrap().trim().clone();

                let index = m * 100 + d;

                let val = map.get(&index);
                if val != None && val.unwrap() != "" {
                    println!("{}-{} : {} and {}", m, d, val.unwrap(), &name);
                }
                map.insert(index, String::from(name));
            }
        }
    }
}
