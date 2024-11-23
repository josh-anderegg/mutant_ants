use super::*;
pub struct Himmelblau;
impl Function for Himmelblau {
    
    fn minimum(&self) -> f64 {
        0.0
    }

    fn domain(&self) -> Domain {
        [[-5.0, 5.0], [-5.0, 5.0]]
    }

    fn eval(&self, p : Point) -> Option<f64> {
        if  self.domain_check(p) {
            let val = (p.0.powi(2) + p.1 -11.0).powi(2) + (p.0 + p.1.powi(2) - 7.0).powi(2);
            Some(val)
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        "himmelblau"
    }
    
}

