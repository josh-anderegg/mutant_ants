mod worker;
mod genes;

use worker::Worker;
use rand::Rng;
use crate::functions::{Point, Function};
use self::genes::Genes;
use std::sync::{Arc, Mutex};

pub struct Colony {
    workers : Vec<Worker>,
    bulletin : Arc<Mutex<(Point, f64)>>
}

impl Colony {
    pub fn solve(&mut self, max_iterations : usize) {
        for _ in 0..max_iterations{
            self.iterate()
        }    
    }

    fn iterate(&mut self) {
        self.workers.iter_mut()
            .for_each(|worker|{
                worker.iterate()
            });
    }

    pub fn new(pop_count : usize, function : &'static dyn Function) -> Colony {
        let mut workers = Vec::new();
        let mut rng = rand::thread_rng();
        let [[x_min, x_max], [y_min, y_max]] = function.domain();
        let colony_gene_pool = Genes::new(&mut rng);
        let colony_center = (rng.gen_range(x_min..=x_max), rng.gen_range(y_min..=y_max));
        let center_val = function.eval(colony_center).unwrap(); 
        let bulletin = Arc::new(Mutex::new((colony_center, center_val)));
        for _ in 0..pop_count {
            workers.push(Worker::new(colony_center, &mut rng, 0.001, function, &colony_gene_pool, Arc::clone(&bulletin)));
        }
        Colony {workers, bulletin}
    }

    pub fn get_best(&self) -> (Point, f64) {
        self.workers.iter()
            .map(|worker| worker.pos_val())
            .min_by(|a,b| a.1.total_cmp(&b.1))
            .unwrap_or(((0.0,0.0),f64::INFINITY))
    }
}