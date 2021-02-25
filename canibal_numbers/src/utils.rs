pub mod dumb_shit{
    pub fn parse_u64(s:&str)->u64{
        let wasnt = "Parameter wasn't a number, oughta be";
        return s.parse().expect(wasnt);
    }
    pub fn dump_vec(v:&Vec<u64>){
        for vv in v{
            print!("{}-",vv);
        }
        print!("\n");
    }
    /**
    * Seek Forwad Find Duplicates
    */
    pub fn slash_forward_find_duplicates(arr:&Vec<u64>,i:usize,len:usize,v:u64)->u64{
       let mut count = 0;
       for j in i..len{
           //println!("{}",arr[j]);
           if arr[j] != v {
               break;
           }
           count +=1;
       }
       return count;
   }
}
//hmm :)
pub fn other_fun()->u64{
    return 4;
}