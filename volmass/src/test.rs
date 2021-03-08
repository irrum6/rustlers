#[cfg(test)]
pub mod tests{
    use crate::shapes::Sphere as Sphere;
    use crate::shapes::VolumeAndMass;
    #[test]
    pub fn test_volume(){
        let s = Sphere::new(8.0);
        let v = s.volume();
        assert_eq!(v.round(),2145.0)
    }
    #[test]
    fn test_mass() {
        let s = Sphere::new(8.0);
        let m = s.mass(7.84);
        assert_eq!(m.round(),16814.0)
    }
}