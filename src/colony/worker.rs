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
        // General direction into which the ant tries to migrate
        let tendency = self.get_tendency(gossip);

        // Some randomization to determine the definite direction the ant will take
        let step = self.determine_direction(tendency);

        // Determine next position based on the step calculated above
        let next_position = ((self.position.0 + step.0 + self.momentum.0) * self.genes.stride, 
                                         (self.position.1 + step.1 + self.momentum.1) * self.genes.stride);
        if self.function.domain_check(next_position) {
            self.momentum = (next_position.0 - self.position.0, next_position.1 - self.position.1);
            let val = self.function.eval(next_position).unwrap();
            self.value = val;
            self.position = next_position;
            return (Action::Move(prev_position, next_position), self.value);
        } else if self.remaining_age > 0{
            self.remaining_age -= 1;
            return (Action::Stall(self.position), self.value);
        } else {
            return (Action::Die(self.position), self.value)
        }
    }


    fn get_tendency(&self, gossip: &(Point, f64)) -> Point {
        // If the gradient for the given function is defined use the gradient, otherwise assume gradient is zero
        let gradient = match self.function.gradient(self.position) {
            Some(gradient) => gradient,
            None => [0.0, 0.0],
        };
        
        // See if the currently best value is worse than our own best value
        // If yes, write our value into the register and have no envy
        // If no, the envy is the direction towards the best value
        let envy = match gossip.1 < self.value {
            true => [0.0, 0.0],
            false => [gossip.0.0 - self.position.0, gossip.0.1 - self.position.1],
        };

        // The envy is combined with the gradient to return the tendency of our worker
        (envy[0] - gradient[0], envy[1] - gradient[1])
    }

    fn determine_direction(&self, tendency: Point) -> Point {
        let mut rng = thread_rng();
        
        // Randomly decide based on the jealousy in the genes of the worker if the worker will follow the tendency, 
        // or if the worker goes into a random direction instead
        if rng.gen_range(0.0..1.0) < self.genes.jealousy {
            tendency
        } else {
            let norm = [norm(tendency), 1.0].into_iter()
                .max_by(|a, b| a.total_cmp(&b)).unwrap();
        
            let random_direction = (rng.gen_range(-norm..norm), rng.gen_range(-norm..norm));
            // Additional model
            // [random_direction[0] + tendency[0], random_direction[1] + tendency[1]]
            // Total randomness
            random_direction
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

fn norm(point: Point) -> f64 {
    (point.0.powf(2.0) + point.1.powf(2.0)).sqrt()
} 

