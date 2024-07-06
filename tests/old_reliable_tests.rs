#[cfg(test)]
const COLONY_COUNT: usize  = 10;
const COLONY_SIZE: usize = 20;
const EPSILON: f64 = 1e-6; // Small epsilon onto which we desire accuracy
const MAX_ITERATIONS: usize = 100_000;

use mutant_ants::{find_minimum, functions::{Ackley, Parabolla, Rastrigin, Rosenbrock, Function}};

fn solution_diff(target: f64, solution: f64) -> f64 {
    (target - solution).abs()
}

#[test]
fn single_colony_parabolla(){
    let solution = find_minimum(&Parabolla, 1, 10, MAX_ITERATIONS, false, EPSILON);
    let target = Parabolla.minimum();
    let diff = solution_diff(target, solution.1);
    println!("{target:?} {solution:?} {diff}");        
    assert!(diff <= EPSILON)
}

#[test]
fn parabolla() {
    let solution = find_minimum(&Parabolla, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,false, EPSILON);
    let target = Parabolla.minimum();
    let diff = solution_diff(target, solution.1);
    println!("{target:?} {solution:?} {diff}");        
    assert!(diff <= EPSILON)
}

#[test]
fn rosenbrock() {
    let solution = find_minimum(&Rosenbrock, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,false, EPSILON);
    let target = Rosenbrock.minimum();
    let diff = solution_diff(target, solution.1);
    println!("{target:?} {solution:?} {diff}");        

    assert!(diff <= EPSILON)
}

#[test]
fn ackley(){
    let solution = find_minimum(&Ackley, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,false, EPSILON);
    let target = Ackley.minimum();
    let diff = solution_diff(target, solution.1);
    println!("{target:?} {solution:?} {diff}");        
    assert!(diff <= EPSILON)
}

#[test]
fn rastrigin() {
    let solution = find_minimum(&Rastrigin, COLONY_COUNT, COLONY_SIZE, MAX_ITERATIONS,false, EPSILON);
    let target = Rastrigin.minimum();
    let diff = solution_diff(target, solution.1);
    println!("{target:?} {solution:?} {diff}");        
    assert!(diff <= EPSILON)
}
