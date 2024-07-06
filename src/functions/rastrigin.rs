use super::*;
use std::f64::consts::PI;
pub struct Rastrigin;
impl Function for Rastrigin {
    
    fn minimum(&self) -> f64 {
        0.0
    }

    fn domain(&self) -> Domain {
        [[-5.12, 5.12], [-5.12, 5.12]]
    }

    fn eval(&self, p : Point) -> Option<f64> {
        // 20 + x^2 + y^2 - 10 (cos(2PIx) + cos(2PIy))
        if  self.domain_check(p) {
            let val = 20.0 + (p.0 * p.0) + (p.1 * p.1) - (10.0 * ((2.0*PI*p.0).cos() + (2.0*PI*p.1).cos())) ;
            Some(val)
        } else {
            None
        }
    }

   
    fn gradient(&self, p : Point) -> Option<(f64, f64)> {
        if self.domain_check(p){
            let dx = 2.0 * p.0 + 20.0 * PI * (2.0 * PI * p.0).sin();
            let dy = 2.0 * p.1 + 20.0 * PI * (2.0 * PI * p.1).sin(); 
            Some((dx,dy))
        } else {
            None
        }
        
    }

    fn name(&self) -> &str {
        "rastrigin"
    }
    
}
