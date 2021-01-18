struct RNG {
    w:u32,
    x:u32,
    y:u32,
    z:u32,
}

impl RNG {
    fn new ()->RNG {
        RNG {
            w:0,
            x:0,
            y:0,
            z:0
        }
    }

    fn seed(& mut self){
        use std::time::{SystemTime, UNIX_EPOCH};
        self.w = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
        self.x = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
        self.y = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
        self.z = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    }

    fn get (& mut self)->u32{
        let tmp:u32 = self.x^(self.x<<15);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w^(self.w>>21))^(tmp^(tmp>>4));
        self.w//this returns
    }
}
enum Values {
    Half = 2147483647,
    ThreeQuarters = 3221225469,
    Quarter = 1073741823,
    Third = 1431655765,
    TwoThirds = 2863311530
}
fn indexes(x:usize,y:usize,value:u32)->(usize,usize){
    if (x==y && x==0)||(x==y && x==29)||(x==0 && y==29)|| (x==29 && y==0) {
        return corner_hop(x,y,value);
    }
    if x==0 || x==29 || y==0 || y==29 {
        return edge_hop(x, y, value);
    }
    normal_hop(x,y,value)
}
fn edge_hop(x:usize,y:usize,value:u32)->(usize,usize){
    //additional safety
    if (x==y && x==0)||(x==y && x==29)||(x==0 && y==29)|| (x==29 && y==0) {
        return corner_hop(x,y,value);
    }
    //now with that
    let mut xyz:[(usize,usize);3]=[(0,1),(0,0),(1,0)];

    if x==0 || x==29 {
        xyz[0] = (x,y+1);
        xyz[1] = (x,y-1);
        let subx = if x==0 {1}else{28};
        xyz[2] = (subx,y);  
    }
    
    if y==0 || y==29{
        xyz[1] = (x+1,y);
        xyz[2] = (x-1,y);
        let suby= if y==0 {1}else{28};
        xyz[0] = (x,suby);          
    }
    if value > Values::TwoThirds as u32 {
        return xyz[0];
    }
    if value > Values::Third as u32{
        return xyz[1];
    }
    return xyz[2]; 
}
fn corner_hop(x:usize,y:usize,value:u32)->(usize,usize){
    let mut xyz:[(usize,usize);2]=[(0,1),(1,0)];
    
    if x==y && x==29 {
        xyz[0] = (28,29);
        xyz[1] = (29,28);
    }
    if x==0 && y==29 {
        xyz[0] = (1,29);
        xyz[1] = (0,28);
    }
    if x==29 && y==0 {
        xyz[0] = (29,1);
        xyz[1] = (28,0);
    }

    if value >Values::Half as u32 {
        return xyz[0];
    }
    return xyz[1];
}
fn normal_hop(x:usize,y:usize,value:u32)->(usize,usize){
    if value > Values::ThreeQuarters as u32 {
        return (x-1,y);
    }
    if value >Values::Half as u32 {
        return (x+1,y)
    }
    if value >Values::Quarter as u32  {
        return (x,y+1);
    };
    return (x,y-1);
}

fn single_round(mut xor:RNG)->(u32,RNG){
    let count = 50;//50 bells

    let mut fleas_grid:[[u8;30];30] = [[1;30];30];
    for _i in 1..=count {
        for i in 0..30{
            for j in 0..30 {
                if fleas_grid[i][j]==0 {                
                    continue;
                }
                fleas_grid[i][j] -= 1;
                let value = xor.get();
                let val_indexes:(usize,usize) = indexes(i, j, value);            
                let ix = val_indexes.0;
                let iy = val_indexes.1;            
                fleas_grid[ix][iy] +=1;     
            }
        } 
    }
    let mut result = 0;
    for i in 0..30{
        for j in 0..30{
            if fleas_grid[i][j]==0 {                
                result +=1;
            }
        }
    }
    (result,xor)
}

fn main() {    
    let mut xor = RNG::new();
    xor.seed();
    let count = 2000;
    let mut result = 0;
    let mut min = 900;
    let mut max = 0;
    //run for x times and count average
    for _i in 1..=count {
        let result_tuple = single_round(xor);
        let zero = result_tuple.0;
        result +=  zero;
        if zero > max{
            max = zero;
        }
        if zero < min{
            min = zero;
        }
        // println!("iteration N{}:result is {}",_i, zero);
        xor = result_tuple.1;
    }
    println!("min:{} , max:{}, average result is {}",min, max, result as f64 / count as f64);
}
