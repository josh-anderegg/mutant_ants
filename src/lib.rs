mod functions;
mod colony;
mod draw;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use colony::Colony;
use draw::draw_history;
use functions::{Function, Point};
use colony::worker::Action;
use colony::ColonyHistory;
type History = Vec<ColonyHistory>; // [colony_id][iteration][points of workers]

fn export_history(history: &History, time: &String) {
    let mut log_file = File::create(format!("logs/{time}.txt")).unwrap();
    for (colony_nr, colony) in history.iter().enumerate(){
        log_file.write(format!("Colony #{}:\n", colony_nr).as_bytes()).unwrap();
        for (iteration_nr, iteration) in colony.iter().enumerate(){
            log_file.write(format!("\tIteration #{}\n", iteration_nr).as_bytes()).unwrap();
            for (worker_id ,worker) in iteration {
                let event = match worker {
                    Action::Born(position) => format!("\t\t#{} was born at {:?}\n", worker_id, position),
                    Action::Stall(position) => format!("\t\t#{} stalled at {:?}\n", worker_id, position),
                    Action::Move(from, to) => format!("\t\t#{} moved from {:?} to {:?}\n", worker_id, from, to),
                    Action::Die(position) => format!("\t\t#{} died at {:?}\n", worker_id, position),
                    
                };
                log_file.write(event.as_bytes()).unwrap();
            }
        }
    }
}


pub fn find_minimum(function: &'static dyn Function, colony_nr: usize, colony_size: usize, max_iterations: usize, track: bool) -> (Point, f64){

    let mut handles = Vec::new();
    let mut colonies = Vec::new();
    let history = if track {
        Some(Arc::new(Mutex::new(History::with_capacity(colony_nr))))
    } else {
        None
    };

    for colony_id in 0..colony_nr {
        let colony = Colony::new(colony_id, colony_size, function);
        let colony_reference = Arc::new(Mutex::new(colony));
        let history = history.clone();
        
        let handle = {
            let colony_reference = Arc::clone(&colony_reference);

            thread::spawn(move || {
                let mut colony = colony_reference.lock().unwrap();
                let colony_history = colony.solve(max_iterations, track);
                if track {
                    if let Some(history) = history {
                        let mut history = history.lock().unwrap();
                        history.push(colony_history.unwrap());
                    }
                    
                }
        })};
        
        handles.push(handle);
        colonies.push(colony_reference);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    if track {

        if let Some(history) = history {
            let history = history.lock().unwrap();
            let now = chrono::Local::now();
            let time_str = now.format("%Y%m%d_%H%M%S").to_string();
            export_history(&history, &time_str);
            draw_history(function, &history, max_iterations, draw::Theme::Neon, &time_str)
        }

        
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
        let solution = find_minimum(&Ackley, 1, 10, 20, true);
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