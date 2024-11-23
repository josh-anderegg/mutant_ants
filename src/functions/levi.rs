use super::*;
use std::f64::consts::PI;

pub struct Levi;
impl Function for Levi {
    
    fn minimum(&self) -> f64 {
        0.0
    }

    fn domain(&self) -> Domain {
        [[-10.0, 10.0], [-10.0, 10.0]]
    }

    fn eval(&self, p : Point) -> Option<f64> {
        if  self.domain_check(p) {
            let first = (3.0 * PI * p.0).sin().powi(2) + (p.0-1.0).powi(2) * (1.0 + (3.0 * PI * p.1).sin().powi(2));
            let second = (p.1 - 1.0).powi(2) * (1.0 + (2.0 * PI * p.1).sin().powi(2));
            let val = first + second;
            Some(val)
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        "levi"
    }
    
}

