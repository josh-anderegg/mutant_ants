use std::sync::{Arc, Mutex};

use crate::functions::{Function, Point};
use super::genes::Genes;
use rand::{rngs::ThreadRng, thread_rng, Rng};
pub struct Worker {
    position : Point,
    genes : Genes,
    function : &'static dyn Function,
    bulletin : Arc<Mutex<(Point, f64)>>,
    prev_val : f64,
    cur_val : f64
}

impl Worker {
    pub fn iterate(&mut self) {
        let tendency = self.get_tendency();
        let step = self.determine_direction(tendency);
        let next_position = (self.position.0 + step[0], self.position.1 + step[1]);
        if self.function.domain_check(next_position) {
            self.prev_val = self.cur_val;
            let val = self.function.eval(next_position).unwrap();
            self.cur_val = val;
            self.position = next_position
        }
    }

    fn get_tendency(&self) -> [f64;2] {
        let gradient = match self.function.gradient(self.position) {
            Some(gradient) => gradient,
            None => [0.0, 0.0],
        };
        let gossip = self.bulletin.lock().unwrap();
        let envy = [gossip.0.0 - self.position.0, gossip.0.1 - self.position.1];
        Worker::normalize([envy[0] - gradient[0], envy[1] - gradient[1]])
    }

    fn determine_direction(&self, orientation : [f64;2]) -> [f64; 2] {
        let mut rng = thread_rng();

        if rng.gen_range(0.0..1.0) < self.genes.jealousy {
            orientation
        } else {
            let random_direction = [rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)];
            Worker::normalize([random_direction[0] + orientation[0], random_direction[1] + orientation[1]])
        }
    }

    fn normalize(vec : [f64 ;2]) -> [f64 ;2] {
        let sum : f64 = vec.iter().sum();
        vec.map(|val| val / sum);
        vec
    }

    pub fn pos_val(&self) -> (Point, f64) {
        (self.position, self.cur_val)
    }

    pub fn new(colony_center : Point, rng : &mut ThreadRng, spray : f64, function : &'static dyn Function, colony_gene_pool : &Genes, bulletin : Arc<Mutex<(Point, f64)>>) -> Worker {
        let x_spray = rng.gen_range(-spray..=spray);
        let y_spray = rng.gen_range(-spray..=spray);
        let start_position = (colony_center.0 + x_spray, colony_center.1 +  y_spray);
        let start_val = match function.eval(start_position) {
            Some(val) => val,
            None => std::f64::INFINITY,
        };
        Worker {
            position: start_position,
            genes: colony_gene_pool.mutate(rng),
            function,
            bulletin,
            prev_val: start_val,
            cur_val: start_val
        }
    }
}



