pub mod leet{
    // use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn nano()->u32{
        // returns 
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos()
    }

    use std::collections::HashMap;
    #[derive(Debug)]
    pub struct TableEntry{
        character:String,
        substitutes:Vec<String>
    }
    impl TableEntry{
        pub fn new(c:&str)->TableEntry{
            let v:Vec<String> = Vec::new();
            TableEntry{character:String::from(c),substitutes:v}
        }
        pub fn add(&mut self,s:&str){
            let c = String::from(s).replace("\"","");
            self.substitutes.push(c);
        }
        pub fn set_character(&mut self,s:&str){
            let c = String::from(s).replace("\"","");
            self.character = c;
        }
        pub fn rand(&self)->String{
            let x:usize = nano() as usize % self.substitutes.len();
            return self.substitutes[x].clone();
        }
    }
    pub struct TableOfSubstitions{
        list:HashMap<String,TableEntry>
    }
    impl TableOfSubstitions {
        pub fn new()->TableOfSubstitions{
            let h:HashMap<String,TableEntry> = HashMap::new();
            TableOfSubstitions{list:h}
        }
        pub fn add(&mut self,te:TableEntry){
            self.list.insert(te.character.clone(),te);
        }
        pub fn get(&self,ss:String)->String{
            let s =self.list.get(&ss);
            if s.is_some() {
                return s.unwrap().rand();                
            }
            return ss;
        }
        pub fn _print(&self){
            for l in &self.list{
                print!("{:?}",l);
            }
        }
    }
}