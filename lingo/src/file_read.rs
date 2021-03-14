pub mod fs1{
    use crate::Lingo;

    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    //copied from rust site, if you don't undestand this don't worry , so did I.
    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
    }
    /**
     * receives string literal as file name, opens read, and then fills list
     */
    pub fn read_dictionary(s:&str, lingo: &mut Lingo){
        if let Ok(lines) = read_lines(s) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    let stronk = String::from(ip.trim());
                    lingo.add(stronk);
                }
            }
        }
    }
}