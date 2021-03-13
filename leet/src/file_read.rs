pub mod fs1{
    use crate::leet::leet::TableEntry as TableEntry;
    use crate::leet::leet::TableOfSubstitions as TableOfSubstitions;

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
     * receives string literal as file name, opens read, and
     * then fills tables
     */
    pub fn read_file_and_fill(s:&str, tos: &mut TableOfSubstitions){
        if let Ok(lines) = read_lines(s) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {                
                    let mut te = TableEntry::new("a");
                    let mut i = 0;
                    let chazar = ip.split(",");                
                    for chaz in chazar{
                        if i == 0 {                        
                            te.set_character(chaz);
                            i +=1;
                            continue;
                        }
                        te.add(chaz);                    
                        i +=1;
                    }
                    tos.add(te);
                }
            }
        }
    }
}