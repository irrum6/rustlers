mod shape;
use shape::shapes as shapes;
use std::env;

mod test;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("pass enough parameters to calculate");
        return; 
    }

    let param1:f64 = args[2].trim().parse().expect("number");
    let param2:f64 = args[3].trim().parse().expect("number");

    let shape = shapes::make_shape(args[1].as_ref(),param1,param2);

    println!("This {} has a volume of {}", shape.name(),shape.volume());

    if args.len() > 4 {
        let density:f64 = args[4].trim().parse().expect("number");
        println!("This {} has a mass of {}", shape.name(),shape.mass(density));
    }
}
