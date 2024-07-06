use super::*;

pub struct Schaffer4;
impl Function for Schaffer4 {
    
    fn minimum(&self) -> f64 {
        0.292579
    }

    fn domain(&self) -> Domain {
        [[-100.0, 100.0], [-100.0, 100.0]]
    }

    fn eval(&self, p : Point) -> Option<f64> {
        if  self.domain_check(p) {
            let first = (p.0.powi(2) - p.1.powi(2)).abs().sin().cos().powi(2) - 0.5;
            let second = (1.0 + 0.001*(p.0.powi(2) + p.1.powi(2))).powi(2);
            let val = 0.5 + (first/second);
            Some(val)
        } else {
            None
        }
    }
    
    fn name(&self) -> &str {
        "schaffer2"
    }
    
}

