use super::*;

pub struct GoldsteinPrice;
impl Function for GoldsteinPrice {
    
    fn minimum(&self) -> f64 {
        3.0
    }

    fn domain(&self) -> Domain {
        [[-2.0, 2.0], [-2.0, 2.0]]
    }

    fn eval(&self, p : Point) -> Option<f64> {
        if  self.domain_check(p) {
            let (x, y) = (p.0, p.1);
            let first = 1.0 + (x + y + 1.0).powi(2) * (19.0 - 14.0 * x + 3.0 * x.powi(2) - 14.0 * y + 6.0 * x * y + 3.0 * y.powi(2));
            let second = 30.0 + (2.0 * x - 3.0 * y).powi(2) * (18.0 - 32.0 * x + 12.0 * x.powi(2) + 48.0 * y - 36.0 * x * y + 27.0 * y.powi(2));
            Some(first * second)
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        "goldstein_price"
    }
    
}

