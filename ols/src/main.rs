// use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod utils;
use utils::utilities::parse_f64 as parse_float;

mod stat;
use stat::statools::*;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut vec1: Vec<f64> = Vec::new();
    let mut vec2: Vec<f64> = Vec::new();
    if let Ok(lines) = read_lines("data.md") {
        // Consumes the iterator, returns an (Optional) String
        let mut i = 0;
        for line in lines {
            if let Ok(ip) = line {
                // println!("{}", ip);
                i += 1;
                if i != 3 && i != 5 {
                    continue;
                }
                let strong = ip.split(",");

                for s in strong {
                    let num = parse_float(s);
                    if i == 3 {
                        vec1.push(num/1000.0);
                    }
                    if i == 5 {
                        vec2.push(num);
                    }
                }
                // println!("{}", ip);
            }
        }
        if vec1.len() != vec2.len() {
            return;
        }
        let b1 = count_b1(&vec1, &vec2);
        let b0 = count_b0(&vec1, &vec2, b1);

        let sse = count_sse(&vec1, &vec2, b1, b0);
        let sst = count_sst(&vec2);

        println!("ნაპოვნია b0:{} + b1:{}x + u=0", b0, b1);

        println!("R2- R კვადრატი ტოლია {}", sse / sst);

        let mut y_hats: Vec<f64> = Vec::new();

        for i in 0..vec1.len() {
            let _y = b0 + b1 * vec1[i];
            y_hats.push(_y);
        }
    }
}
