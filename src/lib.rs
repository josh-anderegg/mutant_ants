mod functions;
mod colony;
mod draw;
use std::sync::{Arc, Mutex};
use std::thread;
use colony::Colony;
use functions::parabolla::Parabolla;
use functions::rosenbrock::Rosenbrock;
use functions::rastrigin::Rastrigin;
use functions::ackley::Ackley;
use functions::{Function, Point};

pub enum FunctionType {
    Rosenbrock,
    Parabolla,
    Ackley,
    Rastrigin
}

struct History {
    data : Vec<Vec<Vec<(usize, Point)>>>, // [colony_id][iteration][points of workers]
}

impl History {
    fn track(&mut self, colony_id : usize, iteration : usize, worker_id : usize,  position : Point) {
        self.data.get_mut(colony_id).unwrap()
                 .get_mut(iteration).unwrap()
                 .push((worker_id, position))
    }

    fn new(colony_nr : usize, iteration_nr : usize) -> Self {
        let data = vec![vec![vec![];iteration_nr];colony_nr];
        History{data}
    }
}

pub fn find_minimum(function : FunctionType, colony_nr : usize, colony_size : usize, max_iterations : usize, track : bool) -> (Point, f64){
    let function_obj: &dyn Function = match function {
        FunctionType::Rosenbrock => &Rosenbrock,
        FunctionType::Parabolla => &Parabolla,
        FunctionType::Ackley => &Ackley,
        FunctionType::Rastrigin => &Rastrigin,
        
    };

    let history = Arc::new(Mutex::new(History::new(colony_nr, max_iterations)));
    let mut handles = Vec::new();
    let mut colonies = Vec::new();
    
    for id in 0..colony_nr {
        let colony = Colony::new(id, colony_size, function_obj);
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
        // draw::draw_history(function_obj, &history.lock().unwrap(), max_iterations);
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

    use crate::find_minimum;
    fn solution_diff(target : ((f64, f64), f64), solution : ((f64, f64), f64)) -> f64 {
        vec![(target.0.0 - solution.0.0).abs(), (target.0.1 - solution.0.1).abs(), (target.1 - solution.1).abs()]
        .into_iter().max_by(|a,b|a.total_cmp(&b)).unwrap()
    }

    #[test]
    fn single_colony_parabolla(){
        let solution = find_minimum(crate::FunctionType::Parabolla, 1, 10, 10, true);
        let target = ((0.0,0.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        
        assert!(diff <= EPSILON)
    }

    #[test]
    fn single_colony_ackley() {
        let solution = find_minimum(crate::FunctionType::Ackley, 1, 10, 100,true);
        let target = ((0.0,0.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        
        assert!(diff <= EPSILON)
    }
    #[test]
    fn parabolla() {
        let solution = find_minimum(crate::FunctionType::Parabolla, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,true);
        let target = ((0.0,0.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        
        assert!(diff <= EPSILON)
    }

    #[test]
    fn rosenbrock() {
        let solution = find_minimum(crate::FunctionType::Rosenbrock, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,true);
        let target = ((1.0,1.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        

        assert!(diff <= EPSILON)
    }

    #[test]
    fn ackley(){
        let solution = find_minimum(crate::FunctionType::Ackley, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,true);
        let target = ((0.0,0.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        
        assert!(diff <= EPSILON)
    }

    #[test]
    fn rastrigin() {
        let solution = find_minimum(crate::FunctionType::Rastrigin, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,true);
        let target = ((0.0,0.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        
        assert!(diff <= EPSILON)
    }
}