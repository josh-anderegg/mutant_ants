use criterion::{criterion_group, criterion_main, Criterion};
use functions::{ackley::Ackley, parabolla::Parabolla, rastrigin::Rastrigin, rosenbrock::Rosenbrock, Function};
use mutant_ants::*;
const EPSILON: f64 = 1e-6;

fn calculate_error(result: f64, expected: f64) -> f64 {
    (result - expected).abs()
}


pub fn test_grid(c: &mut Criterion){
    let mut group = c.benchmark_group("param_grid");
    let functions: Vec<&'static dyn Function> = vec![&Rastrigin, &Parabolla, &Ackley, &Rosenbrock];
    let colony_nrs = vec![1, 10, 20];
    let colony_sizes = vec![10, 20, 30];
    let max_iterations = vec![10_000, 50_000, 100_000, 250_000];
    let epsilons = vec![1e-3, 1e-6, 1e-9];
    c.warm_up_time(std::time::Duration::from_millis(100));
    c.nresamples(50);
    for function in functions.iter() {
        for colony_nr in colony_nrs.iter() {
            for colony_size in colony_sizes.iter() {
                for max_iteration in max_iterations.iter() {
                    for epsilons in epsilons.iter() {
                        let mut total = 0;
                        let mut correct = 0;
                        let mut acc_error = 0.0;
                        let min = function.eval(function.minimum()).unwrap();
                        let name = format!("{} colony count: {} colony size: {} max iteration: {} precision: {}", function.name(), colony_nr, colony_size, max_iteration, epsilons); 
                        group.bench_with_input(name, &(function, colony_nr, colony_size, max_iteration, epsilons), 
                        |b, &(function, colony_nr, colony_size, max_iteration, epsilon)| {
                            b.iter(|| {
                                total += 1;
                                let (_, val) = find_minimum(*function, *colony_nr, *colony_size, *max_iteration, false, *epsilon);
                                let error = calculate_error(val, min);
                                if error < EPSILON{
                                    correct += 1;
                                }
                                acc_error += error;
                            });
                        });
                        let accuracy = (correct as f64) / (total as f64);
                        let avg_error = acc_error / (total as f64);
                        println!("Tests completed with accuracy: {accuracy}\nAnd an average deviation from the minima of: {avg_error}");
                    }
                }
            }
        }
    }

    group.finish();
}


criterion_group!(benches, test_grid);
criterion_main!(benches);