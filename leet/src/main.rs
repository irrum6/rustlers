mod file_read;
use file_read::fs1::read_file_and_fill;

mod leet;
use leet::leet::TableOfSubstitions;

use std::io::stdin;

fn main() {
    // File leet_table.rawtext must exist in current path
    let mut tos = TableOfSubstitions::new();
    read_file_and_fill("./leet_table.rawtext", &mut tos);
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