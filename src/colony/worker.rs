use std::sync::{Arc, Mutex};

use crate::functions::{Function, Point};
use super::genes::Genes;
use rand::{rngs::ThreadRng, thread_rng, Rng};
pub struct Worker {
    pub id : usize,
    position : Point,
    genes : Genes,
    function : &'static dyn Function,
    bulletin : Arc<Mutex<(Point, f64)>>,
    momentum : [f64 ;2],
    cur_val : f64
}

impl Worker {
    pub fn iterate(&mut self) {
        // General direction into which the ant tries to migrate
        let tendency = self.get_tendency();

        // Some randomization to determine the definite direction the ant will take
        let step = self.determine_direction(tendency);

        // Determine next position based on the step calculated above
        let next_position = (self.position.0 + step[0] + self.momentum[0], self.position.1 + step[1] + self.momentum[1]);
        if self.function.domain_check(next_position) {
            self.momentum = [next_position.0 - self.position.0, next_position.1 - self.position.1];
            let val = self.function.eval(next_position).unwrap();
            self.cur_val = val;
            self.position = next_position
        }
    }


    fn get_tendency(&self) -> [f64;2] {
        // If the gradient for the given function is defined use the gradient, otherwise assume gradient is zero
        let gradient = match self.function.gradient(self.position) {
            Some(gradient) => gradient,
            None => [0.0, 0.0],
        };
        
        // Get information of where the best ant of your colony is 
        let mut gossip = self.bulletin.lock().unwrap();

        // See if the currently best value is worse than our own best value
        // If yes, write our value into the register and have no envy
        // If no, the envy is the direction towards the best value
        let envy = match gossip.1 < self.cur_val {
            true => {*gossip = self.pos_val();[0.0, 0.0]},
            false => [gossip.0.0 - self.position.0, gossip.0.1 - self.position.1],
        };

        // The envy is combined with the gradient to return the tendency of our worker
        [envy[0] - gradient[0], envy[1] - gradient[1]]
    }

    fn determine_direction(&self, tendency : [f64;2]) -> [f64; 2] {
        let mut rng = thread_rng();
        
        // Randomly decide based on the jealousy in the genes of the worker if the worker will follow the tendency, 
        // or if the worker goes into a random direction instead
        if rng.gen_range(0.0..1.0) < self.genes.jealousy {
            tendency
        } else {
            let norm = [norm(tendency), 1.0].into_iter()
                .max_by(|a, b| a.total_cmp(&b)).unwrap();
        
            let random_direction = [rng.gen_range(-norm..norm), rng.gen_range(-norm..norm)];
            // Additional model
            // [random_direction[0] + tendency[0], random_direction[1] + tendency[1]]
            // Total randomness
            random_direction
        }
    }

    pub fn pos_val(&self) -> (Point, f64) {
        (self.position, self.cur_val)
    }

    pub fn new(id : usize, colony_center : Point, rng : &mut ThreadRng, spray : f64, function : &'static dyn Function, colony_gene_pool : &Genes, bulletin : Arc<Mutex<(Point, f64)>>) -> Worker {
        let x_spray = rng.gen_range(-spray..=spray);
        let y_spray = rng.gen_range(-spray..=spray);
        let start_position = (colony_center.0 + x_spray, colony_center.1 +  y_spray);
        let start_val = match function.eval(start_position) {
            Some(val) => val,
            None => std::f64::INFINITY,
        };
        Worker {
            id,
            position: start_position,
            genes: colony_gene_pool.mutate(rng),
            function,
            bulletin,
            momentum:  [0.0,0.0],
            cur_val: start_val
        }
    }
}

fn norm(vec : [f64; 2]) -> f64 {
    (vec[0].powf(2.0) + vec[1].powf(2.0)).sqrt()
} 

