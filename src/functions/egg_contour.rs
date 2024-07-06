use super::*;

pub struct EggContour;
impl Function for EggContour {
    
    fn minimum(&self) -> f64 {
        0.0
    }

    fn domain(&self) -> Domain {
        [[-512.0, 512.0], [-512.0, 512.0]]
    }

    fn eval(&self, p : Point) -> Option<f64> {
        if  self.domain_check(p) {
            let t = p.1 + 47.0;
            let val = -t * (p.0/2.0 + t).abs().sqrt().sin() - p.0 * (p.0 - t).abs().sqrt().sin() + 959.6407;
            Some(val)
        } else {
            None
        }
    }
    
    fn name(&self) -> &str {
        "egg_contour"
    }
    
}

