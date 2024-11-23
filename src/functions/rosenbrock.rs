use super::*;
pub struct Rosenbrock;
impl Function for Rosenbrock {
    
    fn minimum(&self) -> f64 {
        0.0
    }

    fn domain(&self) -> Domain {
        [[-MAX_DOMAIN, MAX_DOMAIN], [-MAX_DOMAIN, MAX_DOMAIN]]
    }

    fn eval(&self, p : Point) -> Option<f64> {
        if  self.domain_check(p) {
            let val = 100.0 * (p.1 - p.0.powi(2)).powi(2) + (1.0 - p.0).powi(2);
            Some(val)
        } else {
            None
        }
    }

   
    fn gradient(&self, p : Point) -> Option<(f64, f64)> {
        if self.domain_check(p){
            let dx = 400.0 * p.0.powi(3) - 400.0 * p.0 * p.1 + 2.0 * p.0 - 2.0;
            let dy = 200.0 * (p.1 - p.0.powi(2));
            Some((dx, dy))
        } else {
            None
        }
        
    }
    
    fn name(&self) -> &str {
        "rosenbrock"
    }
    
}
