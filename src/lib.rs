pub mod functions;
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
type Solution = (Point, f64, usize);

fn export_history(function: &'static dyn Function, history: &History, time: &String) {
    std::fs::create_dir_all("target/logs").unwrap();
    let mut log_file = File::create(format!("target/logs/{}_{time}.txt", function.name())).unwrap();
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
                    Action::Starve => format!("\t\t#{} is starving\n", worker_id),
                    Action::Reproduce(child_id) => format!("\t\t#{} gave birth to {}\n", worker_id, child_id),
                    
                };
                log_file.write(event.as_bytes()).unwrap();
            }
        }
    }
}


pub fn find_minimum(function: &'static dyn Function, colony_nr: usize, colony_size: usize, max_iterations: usize, track: bool, precision: f64) -> Solution {

    let mut handles = Vec::new();
    let mut colonies = Vec::new();
    let history = if track {
        Some(Arc::new(Mutex::new(History::with_capacity(colony_nr))))
    } else {
        None
    };

    let min = Arc::new(Mutex::new(((0.0, 0.0), f64::MAX, max_iterations)));
    for colony_id in 0..colony_nr {
        let colony = Colony::new(colony_id, colony_size, function, precision);
        let colony_reference = Arc::new(Mutex::new(colony));
        let history = history.clone();
        
        let handle = {
            let colony_reference = Arc::clone(&colony_reference);
            let min = Arc::clone(&min);

            thread::spawn(move || {
                let mut colony = colony_reference.lock().unwrap();
                let (solution, colony_history) = colony.solve(max_iterations, track);
                let mut min = min.lock().unwrap(); 
                if solution.1 <  min.1{
                    *min = solution;
                }
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
            export_history(function, &history, &time_str);
            draw_history(function, &history, max_iterations, &time_str)
        }

        
    }
    let solution = min.lock().unwrap().clone();
    solution

}

