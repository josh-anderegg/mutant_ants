use super::*;

pub struct Parabolla;
const DOMAIN : Domain = [[-MAX_DOMAIN, MAX_DOMAIN], [-MAX_DOMAIN, MAX_DOMAIN]];
const MINIMUM : Point = (0.0, 0.0); 

impl Function for Parabolla {
    
    fn minimum(&self) -> Point {
        MINIMUM
    }

    fn domain(&self) -> Domain {
        DOMAIN
    }

    fn eval(&self, p : Point) -> Option<f64> {
        // x^2 + y^2 
        if  self.domain_check(p) {
            let val = p.0.powf(2.0) + p.1.powf(2.0);
            Some(val)
        } else {
            None
        }
    }

   
    fn gradient(&self, p : Point) -> Option<(f64, f64)> {
        let dx = 2.0 * p.0;
        let dy = 2.0 * p.1;
        Some((dx,dy))
    }
    
    fn name(&self) -> &str {
        "parabolla"
    }
    
}

