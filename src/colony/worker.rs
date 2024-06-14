use crate::functions::{Function, Point};
use super::genes::Genes;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use super::MAX_AGE;
pub struct Worker {
    pub id: usize,
    pub position: Point,
    genes: Genes,
    function: &'static dyn Function,
    momentum: Point,
    pub value: f64,
    pub remaining_age: usize,
}
#[derive(Clone)]
pub enum Action {
    Born(Point),
    Stall(Point),
    Move(Point, Point),
    Die(Point),
    Starve,
    Reproduce(usize)
}

impl Worker {
    pub fn iterate(&mut self, gossip: &(Point, f64)) -> (Action, f64) {
        let prev_position = self.position;

        // Some randomization to determine the definite direction the ant will take
        let step = self.get_step(gossip);

        // Determine next position based on the step calculated above
        let next_position = (self.position.0 + (step.0 + self.momentum.0) * self.genes.stride, 
                                         self.position.1 + (step.1 + self.momentum.1) * self.genes.stride);
        if self.function.domain_check(next_position) {
            self.momentum = (next_position.0 - self.position.0, next_position.1 - self.position.1);
            let val = self.function.eval(next_position).unwrap();
            self.value = val;
            self.position = next_position;
            return (Action::Move(prev_position, next_position), self.value);
        } else if self.remaining_age > 0{
            self.remaining_age -= 1;
            self.genes.stride *= 0.5;
            return (Action::Stall(self.position), self.value);
        } else {
            return (Action::Die(self.position), self.value)
        }
    }

    fn get_step(&self, gossip: &(Point, f64)) -> Point {
        let mut rng = thread_rng();
        let diff = (self.value - gossip.1).abs();
        
        if diff < 1e-10 {
            return self.random_gradient(&mut rng)
        }
        
        let rand = rng.gen_range(0.0..diff)/diff;
        let next = if rand > self.genes.jealousy {
            gossip.0
        } else {
            let gradient = self.random_gradient(&mut rng);
            (self.position.0 + gradient.0, self.position.1 + gradient.1) 
        };
        next
    }
    fn random_gradient(&self, rng: &mut  ThreadRng) -> Point {
        let norm = self.function.range()[1] * self.genes.stride * self.genes.stride;
        if self.genes.jealousy < 0.3 {
            (rng.gen_range(-norm..norm), rng.gen_range(-norm..norm))
        } else{
            match self.function.gradient(self.position) {
                Some(gradient) => gradient,
                None => (rng.gen_range(-norm..norm), rng.gen_range(-norm..norm)),
            }
        }
    }

    pub fn new(id: usize, colony_center: Point, rng: &mut ThreadRng, spray: f64, function: &'static dyn Function, colony_gene_pool: &Genes) -> Worker {
        let x_spray = rng.gen_range(-spray..=spray);
        let y_spray = rng.gen_range(-spray..=spray);
        let mut start_position = (colony_center.0 + x_spray, colony_center.1 +  y_spray);
        
        // If the point is outside of the bounds from the start retry generation
        while !function.domain_check(start_position) {
            let x_spray = rng.gen_range(-spray..=spray);
            let y_spray = rng.gen_range(-spray..=spray);
            start_position = (colony_center.0 + x_spray, colony_center.1 +  y_spray);
        }

        let start_val = match function.eval(start_position) {
            Some(val) => val,
            None => std::f64::INFINITY,
        };

        let starting_age = rng.gen_range(1..MAX_AGE);
        Worker {
            id,
            position: start_position,
            genes: colony_gene_pool.mutate(rng),
            function,
            momentum:  (0.0,0.0),
            value: start_val,
            remaining_age : starting_age,
        }
    }

    pub fn reproduce(&self, id : usize, rng : &mut ThreadRng) -> Worker {
        Worker::new(id,self.position, rng, 0.01, self.function, &self.genes)
    }
}



