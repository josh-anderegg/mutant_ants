use super::*;

pub struct Parabolla;

impl Function for Parabolla {
    
    fn minimum(&self) -> f64 {
        0.0
    }

    fn eval(&self, p : Point) -> Option<f64> {
        // x^2 + y^2 
        if  self.domain_check(p) {
            let val = p.0.powi(2) + p.1.powi(2);
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

