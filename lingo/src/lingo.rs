pub mod lingo {
    use std::time::{SystemTime, UNIX_EPOCH};
    fn nano()->u32{
        // returns 
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos()
    }
    fn next(w:&Vec<char>,ch:char,start:usize)->usize{
        if start > w.len()-1{
            return w.len();
        }
        let mut i:usize = start;
        loop{
            if w[i]==ch{
                return i;
            }
            i += 1;
            if i > w.len()-1{
                break;
            }
        }
        return w.len();
    }
    pub struct Lingo{
        list:Vec<String>,//privet
        current_word:String,
        mode:u8
    }
    impl Lingo {
        pub fn new()->Lingo{
            Lingo{
                list:Vec::new(),
                current_word:String::new(),
                mode:5//default
            }
        }
        pub fn compare(&self,w:&String)->bool{
            if w.len() != self.current_word.len() {
                println!("Different Length, No Match");
                return false;
            }
            let mut found = 0;
            let wc:Vec<char> = w.chars().collect();
            let mut cwc:Vec<char> = self.current_word.chars().collect();

            for i in 0..wc.len(){
                if wc[i] == cwc [i] {
                    print!("[{}]",wc[i]);
                    cwc[i] = '0';
                    found += 1;
                    continue;
                }                
                let mut index:usize = 0;
                loop{
                    index = next(&cwc,wc[i],index);                    
                    if index > cwc.len()-1 {
                        print!("{}",wc[i]);
                        break;
                    }
                    cwc[index] = '0';
                    print!("({})",wc[i]);
                    break;
                }                
            }
            println!("");
            if found == wc.len() {
                return true;
            }
            return false;
        }
        pub fn set_current_word(&mut self){
            let index:usize = nano() as usize % self.list.len();
            let s = self.list[index].clone();
            self.current_word = s;
        }
        pub fn _set_mode(&mut self,m:u8){
            self.mode = m;
        }
        pub fn get_mode(&self)->u8{
            self.mode
        }
        pub fn add(&mut self,s:String){
            if s.len() !=self.mode as usize {
                return;
            }
            self.list.push(s);
        }
        pub fn _print_self(&self){
            for e in &self.list{
                println!("{}",e);
            }
        }
        pub fn print(&self){
            println!("{}",self.current_word);
        }
    }
}