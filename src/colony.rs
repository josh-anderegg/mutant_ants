pub mod worker;
mod genes;

use std::borrow::BorrowMut;

use plotters::series::Histogram;
use worker::{Action, Worker};
use rand::Rng;
use crate::functions::{Point, Function};
use self::genes::Genes;
pub struct Colony {
    id: usize,
    highest_worker_id: usize,
    workers: Vec<Worker>,
    bulletin: (Point, f64),
    graveyard: Vec<Worker>
}
pub type ColonyHistory = Vec<Vec<(usize, Action)>>;


const STARVE_PERCENTAGE : f64 = 0.4; // The worst % of workers may starve
const REPRODUCE_PERCENTAGE : f64 = 0.1; // The best % of workers may reproduce (actually clone)
const MAX_AGE : usize = 20;
const MAX_COLONY_SIZE : usize = 100;
impl Colony {

    pub fn solve(&mut self, max_iterations: usize, track:bool) -> Option<ColonyHistory> {
        let mut history = if track {
            Some(ColonyHistory::with_capacity(max_iterations))
        } else {
            None
        };
        for _ in 0..max_iterations {
            let mut iteration = if track {
                Some(Vec::with_capacity(self.workers.len()))
            } else {
                None
            };
            for worker in self.workers.iter_mut() {
                let (action, value) = worker.iterate(&self.bulletin);
                if value < self.bulletin.1 {
                    self.bulletin = (worker.position, value)
                }
                if track {
                    match &mut iteration {
                        Some(iteration) => iteration.push((worker.id, action)),
                        None => (),
                    }
                }
            }
            if track {
                match &mut history {
                    Some(history) => history.push(iteration.unwrap()),
                    None => (),
                }
            }
        }
        return history

    }

    fn iterate(&mut self) {
        // Sort the workers by their current value
        self.workers.sort_by_key(|worker| worker.value.partial_cmp(&worker.value).unwrap());
        let mut rng = rand::thread_rng();
        let starving_nrs = self.workers.len() - self.lower_bracket();
        let mut new_borns: Vec<Worker> = Vec::new();
        let upper_bracket = self.upper_bracket();
        for (nr, worker) in self.workers.iter_mut().enumerate(){
            worker.iterate(&self.bulletin);
            if nr < upper_bracket {
                let id = self.highest_worker_id;
                self.highest_worker_id += 1;
                let offspring = worker.reproduce(id, &mut rng);
                new_borns.push(offspring);
            } else if nr >= starving_nrs && worker.remaining_age > 0 {
                worker.remaining_age -= 1;
            }
        }

        // Remove dead workers ot the graveyard
        let mut i = 0 ;
        while i < self.workers.len(){
            if self.workers[i].remaining_age == 0 {
                self.graveyard.push(self.workers.remove(i));
            } else {
                i += 1
            }
        }

        // Add newborn workers
        while self.workers.len() < MAX_COLONY_SIZE && new_borns.len() > 0{
            self.workers.push(new_borns.pop().unwrap())
        }
    }

    pub fn new(id: usize, pop_count: usize, function: &'static dyn Function) -> Colony {
        let mut workers = Vec::new();
        let mut rng = rand::thread_rng();
        let [[x_min, x_max], [y_min, y_max]] = function.domain();
        let colony_center = (rng.gen_range(x_min..=x_max), rng.gen_range(y_min..=y_max));
        let center_val = function.eval(colony_center).unwrap(); 
        let bulletin = (colony_center, center_val);
        
        for id in 0..pop_count {
            let worker_genes = Genes::new(&mut rng);
            workers.push(Worker::new(id, colony_center, &mut rng, 10.0, function, &worker_genes));
        }
        Colony {id, highest_worker_id : pop_count -1, workers, bulletin,  graveyard : vec![]}
    }

    fn lower_bracket(&self) -> usize {
        (self.workers.len() as f64 * STARVE_PERCENTAGE) as usize
    }

    fn upper_bracket(&self) -> usize {
        (self.workers.len() as f64 * REPRODUCE_PERCENTAGE) as usize
    }

    pub fn get_best(&self) -> (Point, f64) {
        let all_workers = self.workers.iter().chain(self.graveyard.iter());
        all_workers.into_iter()
            .map(|worker| (worker.position, worker.value))
            .min_by(|a,b| a.1.total_cmp(&b.1))
            .unwrap_or(((0.0,0.0),f64::INFINITY))
    }
}