use rand::{rngs::ThreadRng, Rng};

pub struct Genes {
    pub orientation : f64, // Probabilty that worker turns into another direction
    pub stride : f64, // Distance the worker is willing to travel
    pub jealousy : f64, // How much the worker is getting influenced by envy

}

impl Genes {
    pub fn new(rng : &mut ThreadRng) -> Genes {
        Genes {
            orientation: rng.gen_range(0.0..1.0),
            stride: rng.gen_range(0.0..=50.0),
            jealousy : rng.gen_range(0.0..1.0)
        }
    }

    pub fn mutate(&self, rng : &mut ThreadRng) -> Genes {
        Genes {
            orientation: clamp(self.orientation + rng.gen_range((-0.3)..0.3), 0.0, 1.0),
            stride: clamp(self.stride + rng.gen_range((-0.3)..0.3), 0.0, 50.0),
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