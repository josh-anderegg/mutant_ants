use rand::{rngs::ThreadRng, Rng};

pub struct Genes {
    pub stride: (f64, f64, f64), // By how much the worker mulitplies it's steps
    pub jealousy: f64, // How much the worker is getting influenced by envy
    pub indecisiveness: f64, // How much noise the movement of the worker has
    function_range: f64, // Hidden value to manage the lenght of genes
}

impl Genes {
    pub fn new(rng: &mut ThreadRng, function_range: f64) -> Genes {
        let stride = (rng.gen_range(0.001..1.0), rng.gen_range(1.0..2.0), rng.gen_range(2.0..4.0));
        let jealousy =  rng.gen_range(0.0..1.0);
        let indecisiveness = rng.gen_range(0.0..(function_range/10_000.0));
        Genes {stride , jealousy, indecisiveness, function_range}
    }

    pub fn mutate(&self, rng: &mut ThreadRng) -> Genes {
        let stride = (clamp(self.stride.0 + rng.gen_range((-0.3)..0.3), 0.001, 1.0),
                                       clamp(self.stride.1 + rng.gen_range((-0.3)..0.3), 1.0, 2.0),
                                       clamp(self.stride.2 + rng.gen_range((-0.3)..0.3), 2.0, 4.0));
        let indecisiveness = rng.gen_range(0.0..(self.function_range/10_000.0));
        Genes {
            stride,
            jealousy: rng.gen_range(0.0..1.0),
            indecisiveness,
            function_range: self.function_range,
        }
    }
}

fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
