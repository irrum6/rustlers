pub mod shapes {
    use std::f64::consts::PI as PI;
    pub trait VolumeAndMass {
        fn volume (&self) ->f64;
        fn mass (&self,density:f64) ->f64;
        fn name (&self)->String;
    }

    pub struct Cylinder{
        radius:f64,
        height:f64
    }
    impl Cylinder {
        pub fn new(r:f64,h:f64)->Cylinder{
        Cylinder{radius:r,height:h}
        }  
    }
    impl VolumeAndMass for Cylinder{
        fn volume(&self)->f64{
            return self.radius*self.radius * self.height * PI;
        }
        fn mass(&self, density:f64) ->f64{
            return self.volume() * density;
        }
        fn name (&self)->String{
            String::from("Cylinder")
        }
    }

    pub struct Cone {
        radius:f64,
        height:f64
    }
    impl Cone{
        pub fn new(r:f64,h:f64)->Cone{
            Cone{radius:r,height:h}
        }   
    }

    impl VolumeAndMass for Cone {
        fn volume(&self)->f64{
            let squared = self.radius *  self.radius ;
            return (PI * squared * self.height ) / 3.0;
        }
        fn mass(&self, density:f64) ->f64{
            return self.volume() * density;
        }
        fn name (&self)->String{
            String::from("Cone")
        }
    }
    pub struct Sphere {
        radius:f64
    }
    impl Sphere {
        pub fn new(r:f64)->Sphere{
            Sphere{radius:r}
        }
    }

    impl VolumeAndMass for Sphere {
        fn volume(&self)->f64{
            let cubed = self.radius *  self.radius *  self.radius;
            return (4.0 * PI * cubed ) / 3.0;
        }
        fn mass(&self, density:f64) ->f64{
            return self.volume() * density;
        }
        fn name(&self)->String{
            String::from("Sphere")
        }
    }
    //o yeah, Boxes
    pub fn make_shape(s:&str,r:f64,h:f64)->Box<dyn VolumeAndMass>{
        let shaper:Box<dyn VolumeAndMass>= match s {
            "s"|"S" =>Box::new(Sphere::new(r)),
            "cc"|"CC" =>Box::new(Cylinder::new(r,h)),
            "c"|"C"|"Co"|"co"|"CO"=>Box::new(Cone::new(r, h)),
            _ =>Box::new(Sphere::new(r))
        };
        shaper
    }

    pub fn print_volume<T: VolumeAndMass>(shape: &T) {
        println!("This {} has an volume of {}", shape.name(),shape.volume());
    }

    pub fn print_mass<T: VolumeAndMass>(shape: T,density:f64) {
        println!("This {} has an mass of {}", shape.name(),shape.mass(density));
    }
}