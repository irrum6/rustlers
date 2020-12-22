mod ftools {
    pub fn lbf_newtons(force: f32, reverse: bool) -> f32 {
        //newton to libre force
        if reverse {
            return (force / 9.81) / 0.453;
        }
        //libre force to newton
        return force * 0.453 * 9.81;
    }
    pub fn ftlb_jouls(energy: f32, reverse: bool) -> f32 {
        //joul to libre foot
        if reverse {
            return ((energy / 9.81) / 0.3048) / 0.453;
        }
        //librefoot to jouls
        return energy * 0.3048 * 0.453 * 9.81;
    }
    pub fn ke_ftlb(grains:f32,velocity:f32) ->f32{
        return (grains * (velocity * velocity)) / 450395.0;
    }
    pub fn ke_si(grams:f32,velocity:f32)->f32 {
        return (grams * (velocity * velocity)) / 2000.0;
    }
    pub fn energy(force: f32, meters: f32) -> f32 {
        return force * meters;
    }

    pub fn force(mass: f32, acceleration: f32) -> f32 {
        return mass * acceleration;
    }
    
    pub fn mass(force: f32, acceleration: f32) -> f32 {
        return force / acceleration;
    }
    
    pub fn acceleration(force: f32, mass: f32) -> f32 {
        return force / mass;
    }
    
    pub fn lb_kg(mass: f32, reverse: bool) -> f32 {
        if reverse {
            return mass / 0.453;
        }
        return mass * 0.453;
    }
    
    pub fn foot_meter(count: f32, reverse: bool) -> f32 {
        if reverse {
            return count / 0.3048;
        }
        return count * 0.3048;
    }
}

#[cfg(test)]
mod tests {
    use crate::ftools::*;
    #[test]
    fn test_force (){
        3;
    }
    #[test]
    fn test_energy () {
        assert_eq!(ke_si(7.45,360.0),482.76);
        assert_eq!(ke_ftlb(230.0,835.0),356.0469);
        assert_eq!(ke_ftlb(230.0,960.0), 470.6269);       
    }
}
fn main() {
    use std::env;

    let args:Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("pass enough parameters to calculate");
        return;
    }

    println!("{}", args[1]);

    let op1: f32 = args[2].trim().parse().expect("type a number");

    let mut op2: f32 = 0.0;
    if args.len() > 3 {
        op2 = args[3].trim().parse().expect("type a number");
    }
    
    let value: f32 = match args[1].as_ref() {
        "force" | "f" => ftools::force(op1, op2),
        "forceN" | "fN" | "fn" => ftools::lbf_newtons(op1,false),
        "forcelb" | "fP" | "fp" => ftools::lbf_newtons(op1, true),
        "energy" | "E" | "e" => ftools::energy(op1, op2),
        "energyJ" | "EJ" | "ej" => ftools::ftlb_jouls(op1, false),
        "energyLFT" | "EP" | "ep" => ftools::ftlb_jouls(op1, true),
        "mass" | "M" | "m" => ftools::mass(op1, op2),
        "acceleration" | "a" => ftools::acceleration(op1, op2),
        "pounds" | "LB" | "lb" | "p" => ftools::lb_kg(op1, false),
        "kilos" | "KG" | "kg" => ftools::lb_kg(op1, true),
        "meters" | "mt" | "MT" => ftools::foot_meter(op1, false),
        "foots" | "ft" | "FT" => ftools::foot_meter(op1, true),
        "kej" | "keJ" => ftools::ke_si(op1, op2),
        "keLb" | "kel" => ftools::ke_ftlb(op1, op2),
        _ => 0.0, //defaultness
    };

    println!("{} is {}", args[1].to_string(), value);
}
