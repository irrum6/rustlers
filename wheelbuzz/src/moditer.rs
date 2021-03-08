pub mod it {
    pub struct FizzBuzzWheel {
        wheel:[u64;7],
        index:usize,
        number:u64,
        bound:u64
    }
    impl FizzBuzzWheel {
        pub fn new(b:u64)->FizzBuzzWheel{
            FizzBuzzWheel{wheel:[3,2,1,3,1,2,3],index:0,number:0,bound:b}
        }
    }
    impl Iterator for FizzBuzzWheel {
        type Item = u64;
        fn next(&mut self)-> Option<Self::Item>{
            self.number += self.wheel[self.index];
            self.index += 1;
            if self.index > self.wheel.len()-1 {
                self.index = 0;
            }
            if self.number < self.bound {
                let result:u64 = self.number;
                return Some(result);
            }
            return None;            
        }
    }
}