pub mod worker;
mod genes;

use worker::{Action, Worker};
use rand::Rng;
use crate::functions::Function;
use self::genes::Genes;
use super::Solution;
use worker::Status;

pub struct Colony {
    _id: usize,
    highest_worker_id: usize,
    workers: Vec<Worker>,
    bulletin: Solution,
    max_size: usize
}
pub type ColonyHistory = Vec<Vec<(usize, Action)>>;


const STARVE_PERCENTAGE : f64 = 0.1; // The worst % of workers may starve
const REPRODUCE_PERCENTAGE : f64 = 0.2; // The best % of workers may reproduce (actually clone)
const MAX_AGE : usize = 20;
impl Colony {

    pub fn solve(&mut self, max_iterations: usize, track:bool) -> (Solution, Option<ColonyHistory>) {
        let mut history = if track {
            let mut history = ColonyHistory::with_capacity(max_iterations);
            let start = self.workers.iter().enumerate()
                .map(|(id, worker)|(id, Action::Born(worker.position)))
                .collect();  
            history.push(start);
            Some(history)

        } else {
            None
        };
        let mut iter_nr = 0;
        while iter_nr < max_iterations{
            let iteration = self.iterate(track);
            if track {
                match &mut history {
                    Some(history) => history.push(iteration.unwrap()),
                    None => (),
                }
            }
            iter_nr += 1;
            if self.bulletin.1 < 1e-15{
                break;
            }
        }
        (self.bulletin, history)
    }

    fn iterate(&mut self, track:bool) -> Option<Vec<(usize, Action)>> {
        // Sort the workers by their current value
        self.workers.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
        let mut rng = rand::thread_rng();
        let starving_nrs = self.workers.len() - self.lower_bracket();
        let mut new_borns: Vec<Worker> = Vec::new();
        let upper_bracket = self.upper_bracket();
        let mut iteration = if track {
            Some(Vec::new())
        } else {
            None
        };

        for (nr, worker) in self.workers.iter_mut().enumerate(){
            if nr < upper_bracket {
                worker.status = Status::Leading;
            } else if nr >= starving_nrs {
                worker.status = Status::Starving;
            } else {
                worker.status = Status::Trailing;
            }

            let (iter_action, value) = worker.iterate(&self.bulletin);

            if value < self.bulletin.1 {
                self.bulletin = (worker.position, value);
            }   

            let hunger_action = if nr < upper_bracket {
                let id = self.highest_worker_id;
                self.highest_worker_id += 1;
                let offspring = worker.reproduce(id, &mut rng);
                new_borns.push(offspring);
                Action::Reproduce(id)
            } else if nr >= starving_nrs && worker.remaining_age > 0 {
                worker.remaining_age -= 1;
                Action::Starve
            } else {
                Action::Die(worker.position)
            };

            if track {
                match &mut iteration {
                    Some(iteration) => {
                        match iter_action {
                            Action::Die(_) => iteration.push((worker.id, iter_action)),
                            _ => {iteration.push((worker.id, iter_action));iteration.push((worker.id, hunger_action))}
                        }
                    },
                    None => (),
                }
            }
        }
        
        // Remove dead workers
        let mut i = 0 ;
        while i < self.workers.len(){
            if self.workers[i].remaining_age == 0 {
                self.workers.remove(i);
            } else {
                i += 1
            }
        }
        
        // Add newborn workers
        while self.workers.len() < self.max_size && new_borns.len() > 0{
            self.workers.push(new_borns.pop().unwrap())
        }
        iteration
    }

    pub fn new(_id: usize, pop_count: usize, function: &'static dyn Function) -> Colony {
        let mut workers = Vec::new();
        let mut rng = rand::thread_rng();
        let [[x_min, x_max], [y_min, y_max]] = function.domain();
        let colony_center = (rng.gen_range(x_min..=x_max), rng.gen_range(y_min..=y_max));
        let function_range = function.range()[1] - function.range()[0];
        for id in 0..pop_count {
            let worker_genes = Genes::new(&mut rng, function_range);
            workers.push(Worker::new(id, colony_center, &mut rng, function, &worker_genes));
        }
        Colony {_id, highest_worker_id : pop_count -1, workers, bulletin: ((f64::MAX, f64::MAX), f64::MAX), max_size: pop_count}
    }

    fn lower_bracket(&self) -> usize {
        (self.workers.len() as f64 * STARVE_PERCENTAGE) as usize
    }

    fn upper_bracket(&self) -> usize {
        (self.workers.len() as f64 * REPRODUCE_PERCENTAGE) as usize
    }

}