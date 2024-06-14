use super::*;
pub struct Rosenbrock;

const DOMAIN : Domain = [[-MAX_DOMAIN, MAX_DOMAIN], [-MAX_DOMAIN, MAX_DOMAIN]];
const MINIMUM : Point = (1.0, 1.0); 

impl Function for Rosenbrock {
    
    fn minimum(&self) -> Point {
        MINIMUM
    }

    fn domain(&self) -> Domain {
        DOMAIN
    }

    fn eval(&self, p : Point) -> Option<f64> {
        if  self.domain_check(p) {
            let val = 100.0 * (p.1 - p.0.powf(2.0)).powf(2.0) + (1.0-p.0).powf(2.0);
            Some(val)
        } else {
            None
        }
    }

   
    fn gradient(&self, p : Point) -> Option<[f64;2]> {
        if self.domain_check(p){
            let dx = 400.0 * p.0.powf(3.0) - 400.0 * p.0 * p.1 + 2.0 * p.0 -2.0;
            let dy = 200.0 * (p.1 - p.0.powf(2.0));
            Some([dx, dy])
        } else {
            None
        }
        
    }
    
    fn name(&self) -> &str {
        "rosenbrock"
    }
    
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn min() {
        let rose = Rosenbrock;
        assert_eq!(rose.eval(rose.minimum()).unwrap(), 0.0)
    }

    #[test]
    fn gradient_min() {
        let rose = Rosenbrock;
        assert_eq!(rose.gradient(rose.minimum()).unwrap(), [0.0, 0.0])
    }
}