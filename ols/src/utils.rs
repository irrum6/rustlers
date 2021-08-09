pub mod utilities {
    pub fn parse_f64(s: &str) -> f64 {
        let wasnt = "Parameter wasn't a number, oughta be";
        return s.parse().expect(wasnt);
    }
}
