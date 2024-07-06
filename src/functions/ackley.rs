use super::*;
use std::f64::consts::PI;
use std::f64::consts::E;

pub struct Ackley;
impl Function for Ackley {
    
    fn minimum(&self) -> f64{
        0.0
    }

    fn domain(&self) -> Domain {
        [[-5.0, 5.0], [-5.0, 5.0]]
    }

    fn eval(&self, p : Point) -> Option<f64> {
        // 20 + x^2 + y^2 - 10 (cos(2PIx) + cos(2PIy))
        if  self.domain_check(p) {
            let first = -20.0 * E.powf(-0.2 * (0.5 * (p.0 * p.0 + p.1 * p.1)).sqrt());
            let second = E.powf(0.5 * ((2.0 * PI * p.0).cos() + (2.0 * PI * p.1).cos()));
            let val = first - second + E + 20.0;
            Some(val)
        } else {
            None
        }
    }
    
    fn name(&self) -> &str {
        "ackley"
    }
    
}

