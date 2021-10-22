pub mod my_random {
    pub struct RNG {
        w: u32,
        x: u32,
        y: u32,
        z: u32,
    }
    impl RNG {
        pub fn new() -> RNG {
            RNG {
                w: 0,
                x: 0,
                y: 0,
                z: 0,
            }
        }
        pub fn seed(&mut self) {
            use std::time::{SystemTime, UNIX_EPOCH};
            self.w = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
            self.x = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
            self.y = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
            self.z = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
        }
        pub fn get(&mut self) -> u32 {
            let tmp: u32 = self.x ^ (self.x << 15);
            self.x = self.y;
            self.y = self.z;
            self.z = self.w;
            self.w = (self.w ^ (self.w >> 21)) ^ (tmp ^ (tmp >> 4));
            self.w //this returns
        }
    }
}
