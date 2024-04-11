use super::*;
use std::f64::consts::PI;
pub struct Rastrigin;
const DOMAIN : Domain = [[-5.12, 5.12], [-5.12, 5.12]];
const MINIMUM : Point = (0.0, 0.0); 

impl Function for Rastrigin {
    
    fn minimum(&self) -> Point {
        MINIMUM
    }

    fn domain(&self) -> Domain {
        DOMAIN
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

   
    fn gradient(&self, p : Point) -> Option<[f64;2]> {
        if self.domain_check(p){
            let dx = 2.0 * p.0 + 20.0 * PI * (2.0 * PI * p.0).sin();
            let dy = 2.0 * p.1 + 20.0 * PI * (2.0 * PI * p.1).sin(); 
            Some([dx,dy])
        } else {
            None
        }
        
    }

}

#[cfg(test)]
mod test {
    use crate::functions::rastrigin::Rastrigin;
    use super::Function;

    #[test]
    fn values(){
        let rast = Rastrigin;
        let values = [0.0,20.25, 1.0, 22.25, 4.0, 26.25, 9.0, 32.25, 80.5];
        let results = [(0.0,0.0), (0.0,0.5), (0.0,1.0), (0.0,1.5), (0.0,2.0), (0.0,2.5), (0.0,3.0), (0.0,3.5), (4.5,4.5)]
            .map(|p| rast.eval(p).unwrap());
        assert_eq!(values, results);
    }

    #[test]
    fn min() {
        let rast = Rastrigin;
        assert_eq!(0.0, rast.eval(rast.minimum()).unwrap());
    }

    #[test]
    fn gradient_min(){
        let rast = Rastrigin;
        assert_eq!([0.0, 0.0], rast.gradient(rast.minimum()).unwrap())
    }
}