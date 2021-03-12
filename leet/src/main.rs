mod file_read;
use file_read::fs1::read_lines;

mod leet;
use leet::leet::TableOfSubstitions;
use leet::leet::TableEntry;

use std::io::stdin;

fn main() {
    // File leet_table.rawtext must exist in current path
    let mut tos = TableOfSubstitions::new();
    if let Ok(lines) = read_lines("./leet_table.rawtext") {
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
    //tos.print();
    let mut line = String::new();
    let exit = String::from("exit");
    loop{
        println!(">type a word");
        stdin().read_line(&mut line).unwrap();//hmm

        if line.trim().eq(&exit) {
            println!("exitas ");
            break;
        }
        let split = line.trim().split("");
        for s in split {
            let c = tos.get(String::from(s));
            print!("{}",c);
        }
        println!("");
        line.truncate(0);//oh yeah
    }    
}