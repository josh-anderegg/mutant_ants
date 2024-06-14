use rand::{rngs::ThreadRng, Rng};

pub struct Genes {
    pub cautiousness : bool, // Worker is either cautius stride between 0.0..1.0 or adventorous stride between 1.0..50.0 
    pub stride : f64, // Distance the worker is willing to travel
    pub jealousy : f64, // How much the worker is getting influenced by envy
}

impl Genes {
    pub fn new(rng : &mut ThreadRng) -> Genes {
        let cautiousness = rng.gen_bool(0.5);
        let stride = match cautiousness {
            true => rng.gen_range(0.001..1.0),
            false => rng.gen_range(1.0..=3.0),
        };
        
        let jealousy =  rng.gen_range(0.0..1.0);

        Genes {cautiousness, stride , jealousy}
    }

    pub fn mutate(&self, rng : &mut ThreadRng) -> Genes {
        let cautiousness = rng.gen_bool(0.5);
        let stride = match cautiousness {
            true => clamp(self.stride + rng.gen_range((-0.3)..0.3), 0.001, 1.0),
            false => clamp(self.stride + rng.gen_range((-1.0)..1.0), 0.001, 3.0),
        };
        Genes {
            cautiousness,
            stride,
            jealousy: clamp(self.jealousy + rng.gen_range((-0.3)..0.3), 0.0, 1.0),
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