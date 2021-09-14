// BirthDayProblem
mod bdp_impl {
    const MIN_DAY: u8 = 1;

    const MAX_DAYS: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const OFFSETS: [usize; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
    const MAX_MONTH: u8 = 12;
    const MIN_MONTH: u8 = 1;

    pub struct Student {
        name: String,
        month: u8,
        day: u8,
    }
    impl Student {
        //returns true if violates
        fn check_month_constraint(m: u8) -> bool {
            m < MIN_MONTH || m > MAX_MONTH
        }
        //returns true if violates
        fn check_day_constraint(d: u8, m: u8) -> bool {
            d < MIN_DAY || d > MAX_DAYS[m as usize - 1]
        }
        pub fn new(n: &str, m: u8, d: u8) -> Student {
            if Student::check_month_constraint(m) || Student::check_day_constraint(d, m) || n == ""
            {
                panic!("bad data");
            }
            return Student {
                name: String::from(n),
                month: m,
                day: d,
            };
        }
    }
    pub fn find_and_print_matches(studs: &Vec<Student>) {
        let mut arr: [&str; 365] = [""; 365];

        for st in studs {
            let index = OFFSETS[st.month as usize - 1] + st.day as usize - 1;
            if arr[index] != "" {
                println!("{}-{} : {} and {}", st.month, st.day, arr[index], st.name);
                // continue;
            }
            arr[index] = st.name.as_str();
        }
    }
}

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

use std::env;

use bdp_impl::find_and_print_matches;
use bdp_impl::Student;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].clone();

    let mut v: Vec<Student> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(linevar) = line {
                let mut fields = linevar.split(",");
                let m: u8 = fields.next().unwrap().trim().parse().expect("NaN");
                let d: u8 = fields.next().unwrap().trim().parse().expect("NaN");
                let n = fields.next().unwrap();
                v.push(Student::new(n, m, d));
            }
        }
    }

    find_and_print_matches(&mut v);
}
