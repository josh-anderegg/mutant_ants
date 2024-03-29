pub mod rastrigin;
pub mod ackley;
pub mod rosenbrock;
pub mod parabolla;

pub type Point = (f64, f64);
pub type Domain = [[f64;2];2];
// Set the domain to a million, as otherwise the random function has issues
const MAX_DOMAIN : f64 = 1_000_000_000.0;

pub trait Function: Sync {
    fn minimum(&self) -> Point;
    
    fn domain(&self) -> Domain;
    
    fn eval(&self, p : Point) -> Option<f64>; // Given as matlab uses f64 for number precision
    
    fn gradient(&self, p : Point) -> Option<[f64;2]>;
    
    fn domain_check(&self, p : Point) -> bool {
        let domain = self.domain();
        p.0 >= domain[0][0] && p.0 <= domain[0][1] && p.1 >= domain[1][0] && p.1 <= domain[1][1]
    }

}


