use super::*;
use std::f64::consts::PI;
use std::f64::consts::E;

pub struct Ackley;
const DOMAIN : Domain = [[-5.0, 5.0], [-5.0, 5.0]];
const MINIMUM : Point = (0.0, 0.0); 
const RANGE : [f64; 2] = [0.0, 15.0];
impl Function for Ackley {
    
    fn minimum(&self) -> Point {
        MINIMUM
    }

    fn domain(&self) -> Domain {
        DOMAIN
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

   
    fn gradient(&self, _p : Point) -> Option<[f64;2]> {
        None
    }

    fn range(&self) -> [f64;2] {
        RANGE
    }
    
    fn name(&self) -> &str {
        "ackley"
    }
    
}

