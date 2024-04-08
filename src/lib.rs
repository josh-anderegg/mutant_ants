mod functions;
mod colony;
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


pub fn find_minimum(function : FunctionType, colony_nr : usize, colony_size : usize, max_iterations : usize) -> (Point, f64){
    let function_obj: &dyn Function = match function {
        FunctionType::Rosenbrock => &Rosenbrock,
        FunctionType::Parabolla => &Parabolla,
        FunctionType::Ackley => &Ackley,
        FunctionType::Rastrigin => &Rastrigin,
        
    };

    let mut handles = Vec::new();
    let mut colonies = Vec::new();
    for id in 0..colony_nr {
        let colony = Colony::new(id, colony_size, function_obj);
        let colony_reference = Arc::new(Mutex::new(colony));
        let handle = {
            let colony_reference = Arc::clone(&colony_reference);
            thread::spawn(move || {
                let mut colony = colony_reference.lock().unwrap();
                colony.solve(max_iterations);
        })};
        
        handles.push(handle);
        colonies.push(colony_reference);
    }

    for handle in handles {
        handle.join().unwrap();
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
    const MAX_ITERATIONS : usize = 100_00;

    use crate::find_minimum;

    fn solution_diff(target : ((f64, f64), f64), solution : ((f64, f64), f64)) -> f64 {
        vec![(target.0.0 - solution.0.0).abs(), (target.0.1 - solution.0.1).abs(), (target.1 - solution.1).abs()]
        .into_iter().max_by(|a,b|a.total_cmp(&b)).unwrap()
    }

    #[test]
    fn single_colony(){
        let solution = find_minimum(crate::FunctionType::Parabolla, 1, 10, 100);
        let target = ((0.0,0.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        
        assert!(diff <= EPSILON)
    }
    
    #[test]
    fn parabolla() {
        let solution = find_minimum(crate::FunctionType::Parabolla, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,);
        let target = ((0.0,0.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        
        assert!(diff <= EPSILON)
    }

    #[test]
    fn rosenbrock() {
        let solution = find_minimum(crate::FunctionType::Rosenbrock, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,);
        let target = ((1.0,1.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        

        assert!(diff <= EPSILON)
    }

    #[test]
    fn ackley(){
        let solution = find_minimum(crate::FunctionType::Ackley, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,);
        let target = ((0.0,0.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        
        assert!(diff <= EPSILON)
    }

    #[test]
    fn rastrigin() {
        let solution = find_minimum(crate::FunctionType::Rastrigin, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,);
        let target = ((0.0,0.0), 0.0);
        let diff = solution_diff(target, solution);
        println!("{target:?} {solution:?} {diff}");        
        assert!(diff <= EPSILON)
    }
}