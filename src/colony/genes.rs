use rand::{rngs::ThreadRng, Rng};

pub struct Genes {
    cautiousness: bool, // Worker is either cautius stride between 0.0..1.0 or adventorous stride between 1.0..3.0 
    pub stride: f64, // By how much the worker mulitplies it's steps
    pub jealousy: f64, // How much the worker is getting influenced by envy
    pub indecisiveness: f64, // How much noise the movement of the worker hase
    function_range: f64,
}

impl Genes {
    pub fn new(rng: &mut ThreadRng, function_range: f64) -> Genes {
        let cautiousness = rng.gen_bool(0.5);
        let stride = match cautiousness {
            true => rng.gen_range(0.001..1.0),
            false => rng.gen_range(1.0..=3.0),
        };
        let jealousy =  rng.gen_range(0.0..1.0);
        let indecisiveness = rng.gen_range(0.0..(function_range/1_000.0));
        Genes {cautiousness, stride , jealousy, indecisiveness, function_range}
    }

    pub fn mutate(&self, rng: &mut ThreadRng) -> Genes {
        let cautiousness = rng.gen_bool(0.5);
        let stride = match cautiousness {
            true => clamp(self.stride + rng.gen_range((-0.3)..0.3), 0.001, 1.0),
            false => clamp(self.stride + rng.gen_range((-1.0)..1.0), 0.001, 3.0),
        };
        let indecisiveness = rng.gen_range(0.0..(self.function_range/1_000.0));
        Genes {
            cautiousness,
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