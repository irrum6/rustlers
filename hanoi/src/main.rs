
fn find_rod(arr:&[u8],n:usize,rod:u8)->usize{
    let mut i:usize = 0;
    loop{
        if arr[i]==rod {
            return i;
        }
        i +=1;
        if i>n-1{
            break;
        }
    }
    return n;
}

fn hanoi2(arr:&mut[u8],n:usize){
    let mut stoper = 0;//for extra safety
    let neven = n%2==0;
    let nodd = n%2==1;
    loop{  
        for i in 1..=n{
            let one = find_rod(&arr, n, 1);
            let two = find_rod(&arr, n, 2);
            let three = find_rod(&arr, n, 3);

            let legitone = one > i-1 || one >= n;
            let legittwo = two > i-1 || two >= n;
            let legitthree  = three > i-1 || three >= n;

            let ieven = i%2==0;
            let iodd = i%2==1;
            // 
            let others = find_rod(&arr, n, arr[i-1]);
            let l = i<2 || (others==i-1 || others >=n); //no small disk lied upon current disk
            if (neven && ieven)||(nodd && iodd) {
                //legit moves
                //can't be put on smaller disk
                if arr[i-1]==1 && legitthree &&l {
                    arr[i-1] = 3;
                    println!("disk :{} moves from rod {} to {}",i,1,3);
                    continue;
                }
                if arr[i-1]==2 && legitone  &&l{
                    arr[i-1] = 1;
                    println!("disk :{} moves from rod {} to {}",i,2,1);
                    continue;
                }
                if arr[i-1]==3 && legittwo  &&l{
                    arr[i-1] = 2;
                    println!("disk :{} moves from rod {} to {}",i,3,2);
                    continue;
                }
            }else{
                if arr[i-1]==1 && legittwo  &&l{
                    arr[i-1] = 2;
                    println!("disk :{} moves from rod {} to {}",i,1,2);
                    continue;
                }
                if arr[i-1]==2 && legitthree  &&l{
                    arr[i-1] = 3;
                    println!("disk :{} moves from rod {} to {}",i,2,3);
                    continue;
                }
                if arr[i-1]==3 && legitone  &&l{
                    arr[i-1] = 1;
                    println!("disk :{} moves from rod {} to {}",i,3,1);
                    continue;
                }
            }
        }
        let mut count = 0;
        for i in 0..=n-1{
            print!("{}",arr[i]);
            if arr[i]==3{count +=1;}
        }
        println!("-x-");
        if count==n {
            break;
        }
        stoper += 1;

        if stoper > 1<<n {
            break;
        }
    }
}
#[test]
fn test() {
    // unimplemented!();
    let mut arr:[u8;4]=[1;4];
    hanoi2(&mut arr, 4);
    assert_eq!(arr[0],3);
}

fn main() {
    let mut arr:[u8;3]=[1;3];
    hanoi2(&mut arr, 3);    
    // values
    //1 is on first rod
    //2 is on second rod
    //3 is on third rod
    println!("Hello, world!");
    let mut arr:[u8;4]=[1;4];
    hanoi2(&mut arr, 4);
}
//EXTRA:
//algorithm follows as
//if theres no small disk atop and destination rod is either free or has bigger disk
//then
//if n is odd
//odd numbered disk can move
//from start (1) to goal rod (3)
//from middle (2) to start(1)
//from goal (3) to middle (2)
//even numbered
//from start (1) to middle (2)
//from middle (2) to goal(3)
//from goal(3) to start(1)

//if n is even rules are flipped for even and on numbered disks