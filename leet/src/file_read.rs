pub mod fs1{
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

//copied from rust site, if you don't undestand this don't worry , so did I.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}