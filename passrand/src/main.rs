use std::convert::TryFrom;
use std::convert::TryInto;
use std::env;

struct RNG {
    w: u32,
    x: u32,
    y: u32,
    z: u32,
}

impl RNG {
    fn new() -> RNG {
        RNG {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn seed(&mut self) {
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

    fn get(&mut self) -> u32 {
        let tmp: u32 = self.x ^ (self.x << 15);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 21)) ^ (tmp ^ (tmp >> 4));
        self.w //this returns
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("pass enough parameters to calculate");
        return;
    }
    let amount: u32 = args[1].trim().parse().expect("type a number");

    let mut length: u32 = 12; //default 12 character length password

    if args.len() > 2 {
        length = args[2].trim().parse().expect("type a number");
    }

    let mut rng = RNG::new();
    rng.seed();

    let charski = vec![
        "ა", "ბ", "გ", "დ", "ე", "ვ", "ზ", "თ", "ი", "კ", "ლ", "მ", "ნ", "ო", "პ", "ჟ", "რ", "ს",
        "ტ", "უ", "ფ", "ქ", "ღ", "ყ", "შ", "ჩ", "ც", "ძ", "წ", "ჭ", "ხ", "ჯ", "ჰ", "ა", "ა", "ა",
        "ა", "ე", "ე", "ე", "ი", "ი", "ი", "ო", "ო", "ო", "უ", "უ",
    ];
    let l: u32 = charski.len().try_into().unwrap();

    for _i in 0..amount {
        for _j in 0..length {
            let rand = rng.get();
            let index: u32 = rand % l;
            let n_us = usize::try_from(index).unwrap();
            print!("{}", charski[n_us]);
        }
        print!(":{}\n", _i);
    }
}
