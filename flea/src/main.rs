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
    if x==0 {
        if value > Values::TwoThirds as u32 {
            return (0,y+1);
        }else if value > Values::Third as u32{
            return (0,y-1);
        }else{
            return (1,y);
        }         
    }    
    if x==29 {
        if value > Values::TwoThirds as u32 {
            return (29,y+1);
        }else if value > Values::Third as u32{
            return (29,y-1);
        }else{
            return (28,y);
        }         
    }   
    
    if y==0 {
        if value > Values::TwoThirds as u32 {
            return (x,1);
        }else if value > Values::Third as u32{
            return (x+1,0);
        }else{
            return (x-1,0);
        }         
    }else {
        //y==29 implicit
        if value > Values::TwoThirds as u32 {
            return (x,28);
        }else if value > Values::Third as u32{
            return (x+1,29);
        }else{
            return (x-1,29);
        }         
    } 
}
fn corner_hop(x:usize,y:usize,value:u32)->(usize,usize){
    if x==y && x==0 {
        if value >Values::Half as u32 {
            return (1,0)
        }
        return (0,1);
    }
    if x==y && x==29 {
        if value >Values::Half as u32 {
            return (28,29);
        }
        return (29,28);
    }
    if x==0 && y==29 {
        if value >Values::Half as u32 {
            return (1,29);
        }
        return (0,28);
    }else {
        // x==29 && y==0 implicit
        if value >Values::Half as u32 {
            return (29,1)
        }return (28,0);
    }
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

fn main() {    
    let mut fleas_grid:[[u8;30];30] = [[1;30];30];
    let mut xor = RNG::new();
    xor.seed();

    // 50 rounds
    for _i in 1..=50 {

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
    println!("result is {}",result);
}
