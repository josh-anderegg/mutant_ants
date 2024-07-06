pub mod rastrigin;
pub mod ackley;
pub mod rosenbrock;
pub mod parabolla;
pub mod schaffer4;
pub mod levi;
pub mod egg_contour;
pub mod goldstein_price;
pub mod himmelblau;

pub use rastrigin::Rastrigin;
pub use ackley::Ackley;
pub use rosenbrock::Rosenbrock;
pub use parabolla::Parabolla;
pub use schaffer4::Schaffer4;
pub use levi::Levi;
pub use egg_contour::EggContour;
pub use goldstein_price::GoldsteinPrice;
pub use himmelblau::Himmelblau;

pub type Point = (f64, f64);
pub type Domain = [[f64;2];2];

// Set the domain to a million, as otherwise the random function has issues
const MAX_DOMAIN : f64 = 1_000_000.0;

pub trait Function: Sync {
    fn minimum(&self) -> f64;
    
    fn domain(&self) -> Domain{
        [[-MAX_DOMAIN, MAX_DOMAIN], [-MAX_DOMAIN, MAX_DOMAIN]]
    }
    
    fn eval(&self, p : Point) -> Option<f64>; // Given as matlab uses f64 for number precision
    
    fn gradient(&self, _p : Point) -> Option<Point> {
        None
    }
    
    fn norm(&self) -> f64 {
        let [x_range, _] = self.domain();
        (x_range[0] - x_range[1]).abs()
    }

    fn domain_check(&self, p : Point) -> bool {
        let domain = self.domain();
        p.0 >= domain[0][0] && p.0 <= domain[0][1] && p.1 >= domain[1][0] && p.1 <= domain[1][1]
    }

    fn name(&self) -> &str{
        "unknown"
    }

}


