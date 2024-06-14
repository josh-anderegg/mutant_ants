
#[cfg(test)]
const COLONY_COUNT: usize  = 10;
const COLONY_SIZE: usize = 20;
const EPSILON: f64 = 1e-8; // Small epsilon onto which we desire accuracy
const MAX_DRAW_ITERATIIONS: usize = 500;

use mutant_ants::{find_minimum, functions::{ackley::Ackley, parabolla::Parabolla, rastrigin::Rastrigin, rosenbrock::Rosenbrock}};

fn solution_diff(target : ((f64, f64), f64), solution : ((f64, f64), f64)) -> f64 {
    (target.1 - solution.1).abs()
    }
    
#[test]
#[ignore]
fn draw_rastrigin() {
    let solution = find_minimum(&Rastrigin, COLONY_COUNT, COLONY_SIZE, MAX_DRAW_ITERATIIONS,true);
    let target = ((0.0,0.0), 0.0);
    let diff = solution_diff(target, solution);
    println!("{target:?} {solution:?} {diff}");        
    assert!(diff <= EPSILON)
}

#[test]
#[ignore]
fn draw_rosenbrock() {
    let solution = find_minimum(&Rosenbrock, COLONY_COUNT, COLONY_SIZE, MAX_DRAW_ITERATIIONS,true);
    let target = ((1.0,1.0), 0.0);
    let diff = solution_diff(target, solution);
    println!("{target:?} {solution:?} {diff}");        
    assert!(diff <= EPSILON)
}

#[test]
#[ignore]

fn draw_ackley() {
    let solution = find_minimum(&Ackley, 10, 10, MAX_DRAW_ITERATIIONS, true);
    let target = ((0.0,0.0), 0.0);
    let diff = solution_diff(target, solution);
    println!("{target:?} {solution:?} {diff}");        
    assert!(diff <= EPSILON)
}

#[test]
#[ignore]
fn draw_parabolla() {
    let solution = find_minimum(&Parabolla, 10, 10, MAX_DRAW_ITERATIIONS, true);
    let target = ((0.0,0.0), 0.0);
    let diff = solution_diff(target, solution);
    println!("{target:?} {solution:?} {diff}");        
    assert!(diff <= EPSILON)
}
