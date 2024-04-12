mod functions;
mod colony;
mod draw;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use colony::Colony;
use functions::{Function, Point};
use colony::worker::Action;

struct History {
    data : Vec<Vec<Vec<(usize, Action)>>>, // [colony_id][iteration][points of workers]
}

impl History {
    fn track(&mut self, colony_id : usize, iteration : usize, worker_id : usize,  action : Action) {
        self.data.get_mut(colony_id).unwrap()
                 .get_mut(iteration).unwrap()
                 .push((worker_id, action))
    }

    fn new(colony_nr : usize, iteration_nr : usize) -> Self {
        let data: Vec<Vec<Vec<(usize, Action)>>> = vec![vec![vec![];iteration_nr];colony_nr];
        History{data}
    }

    fn log(&self) {
        let mut log_file = File::create("logs/log.txt").unwrap();
        for (colony_nr, colony) in self.data.iter().enumerate(){
            log_file.write(format!("Colony #{}:\n", colony_nr).as_bytes()).unwrap();
            for (iteration_nr, iteration) in colony.iter().enumerate(){
                log_file.write(format!("\tIteration #{}\n", iteration_nr).as_bytes()).unwrap();
                for (worker_id ,worker) in iteration {
                    let event = match worker {
                        Action::Born(position) => format!("\t\t#{} was born at {:?}\n", worker_id, position),
                        Action::Stall(position) => format!("\t\t#{} stalled at {:?}\n", worker_id, position),
                        Action::Move(from, to) => format!("\t\t#{} moved from {:?} to {:?}\n", worker_id, from, to),
                    };
                    log_file.write(event.as_bytes()).unwrap();
                }
            }
        }
    }
}

pub fn find_minimum(function : &'static dyn Function, colony_nr : usize, colony_size : usize, max_iterations : usize, track : bool) -> (Point, f64){

    let history = Arc::new(Mutex::new(History::new(colony_nr, max_iterations)));
    let mut handles = Vec::new();
    let mut colonies = Vec::new();
    
    for id in 0..colony_nr {
        let colony = Colony::new(id, colony_size, function);
        let colony_reference = Arc::new(Mutex::new(colony));
        let handle = {
            let colony_reference = Arc::clone(&colony_reference);
            let history_reference = Arc::clone(&history);
            thread::spawn(move || {
                let mut colony = colony_reference.lock().unwrap();
                match track {
                    true => colony.solve_and_track(max_iterations, history_reference),
                    false => colony.solve(max_iterations),
                }
                
        })};
        
        handles.push(handle);
        colonies.push(colony_reference);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    if track {
        history.lock().unwrap().log();
        draw::draw_history(function, &history.lock().unwrap(), max_iterations, draw::Theme::Neon);
    }

    colonies.iter()
        .map(|colony| colony.lock().unwrap().get_best())
        .min_by(|a,b| a.1.total_cmp(&b.1))
        .unwrap_or(((0.0,0.0), std::f64::INFINITY))
    
    

}

#[cfg(test)]
mod test {
    const COLONY_COUNT : usize  = 10;
    const COLONY_SIZE : usize = 50;
    const EPSILON : f64 = 1e-50; // Small epsilon onto which we desire accuracy
    const MAX_ITERATIONS : usize = 10_000;

    use crate::{find_minimum, functions::{ackley::Ackley, parabolla::Parabolla, rastrigin::Rastrigin, rosenbrock::Rosenbrock, Function}};
    fn solution_diff(target : ((f64, f64), f64), solution : ((f64, f64), f64)) -> f64 {
        vec![(target.0.0 - solution.0.0).abs(), (target.0.1 - solution.0.1).abs(), (target.1 - solution.1).abs()]
        .into_iter().max_by(|a,b|a.total_cmp(&b)).unwrap()
    }

    #[test]
    fn single_colony_parabolla(){
        let solution = find_minimum(&Parabolla, 1, 10, 10, false);
        let target = ((0.0,0.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        
        assert!(diff <= EPSILON)
    }

    #[test]
    fn single_colony_ackley() {
        let solution = find_minimum(&Ackley, 1, 10, 10, true);
        let target = ((0.0,0.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        
        assert!(diff <= EPSILON)
    }
    #[test]
    fn parabolla() {
        let solution = find_minimum(&Parabolla, COLONY_COUNT, COLONY_SIZE, 10,false);
        let target = ((0.0,0.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        
        assert!(diff <= EPSILON)
    }

    #[test]
    fn rosenbrock() {
        let solution = find_minimum(&Rosenbrock, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,false);
        let target = ((1.0,1.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        

        assert!(diff <= EPSILON)
    }

    #[test]
    fn ackley(){
        let solution = find_minimum(&Ackley, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,false);
        let target = ((0.0,0.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        
        assert!(diff <= EPSILON)
    }

    #[test]
    fn rastrigin() {
        let solution = find_minimum(&Rastrigin, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,false);
        let target = ((0.0,0.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        
        assert!(diff <= EPSILON)
    }
}