pub mod gela {
    pub trait Vehicle {
        fn movement(&self);
    }
    pub enum CarTypes {
        OffRoad,
        Sports,
        Ordinary,
    }
    // base struct
    pub struct Car {
        size: u32,
        breed: CarTypes,
    }
    impl Vehicle for Car {
        fn movement(&self) {
            println!("vroom");
        }
    }
    //super1
    struct SUV {
        car: Car,
    }
    impl Vehicle for SUV {
        fn movement(&self) {
            self.car.movement();
            println!("SUV");
        }
    }
    // super2
    struct Coupe {
        car: Car,
    }
    impl Vehicle for Coupe {
        fn movement(&self) {
            self.car.movement();
            println!("Sports Car");
        }
    }
    impl Car {
        pub fn new(size: u32, breed: CarTypes) -> Car {
            return Car { size, breed };
        }
        pub fn every(size: u32, breed: CarTypes) -> Box<dyn Vehicle> {
            let car: Box<dyn Vehicle> = match breed {
                CarTypes::OffRoad => Box::new(SUV {
                    car: Car { size, breed },
                }),
                CarTypes::Sports => Box::new(Coupe {
                    car: Car { size, breed },
                }),
                CarTypes::Ordinary => Box::new(Car { size, breed }),
                _ => Box::new(Car { size, breed }),
            };
            return car;
        }
    }
}

use gela::{Car,CarTypes::OffRoad as OffRoad, CarTypes::Sports as Sports,CarTypes::Ordinary as Ordinary};

fn main() {
    let jeep = Car::every(6,OffRoad);
    jeep.movement();
    println!("------");
    let lambo = Car::every(4,Sports);
    lambo.movement();
    println!("------");
    let ford = Car::every(6, Ordinary);
    ford.movement();
}
