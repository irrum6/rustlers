mod file_read;
use file_read::fs1::read_dictionary as read_dictionary;

mod lingo;
use lingo::lingo::Lingo as Lingo;

use std::io::stdin;
fn main() {
    let mut lingo = Lingo::new();
    read_dictionary("dictionary.rawtext",&mut lingo);
    //lingo.print_self();
    lingo.set_current_word();

    let mut line = String::new();
    let mut counter = 0;
    loop{
        println!(">type {} letter word", lingo.get_mode());
        stdin().read_line(&mut line).unwrap();//hmm
        if line.trim().eq("surrender") {
            lingo.print();
            break;
        }
        let trimmed = String::from(line.trim());
        let guess = lingo.compare(&trimmed);
        if guess{
            println!("you won!");
            break;
        }
        counter += 1;
        if counter > 12 {
            println!("type surrender to resign game");
        }        
        line.truncate(0);
    }
}
