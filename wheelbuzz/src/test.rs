#[cfg(test)]
pub mod tester{
    use crate::FizzBuzzWheel;
    #[test]
    fn hundred(){
        let mut w = FizzBuzzWheel::new(100);
        assert_eq!(w.next().unwrap(),3);
        assert_eq!(w.next().unwrap(),5);
        assert_eq!(w.next().unwrap(),6);
    }
}