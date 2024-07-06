use criterion::{criterion_group, criterion_main, Criterion};
use functions::*;
use mutant_ants::*;
use std::fs::{self, File};

#[macro_use] extern crate prettytable;
use prettytable::{Table, format};

fn calculate_error(result: f64, expected: f64) -> f64 {
    (result - expected).abs()
}

pub fn test_grid(c: &mut Criterion){
    let functions: Vec<&'static dyn Function> = vec![&Rastrigin, &Parabolla, &Ackley, &Rosenbrock, &EggContour, &GoldsteinPrice, &Himmelblau, &Levi, &Schaffer4];
    let colony_nrs = vec![1, 20];
    let colony_sizes = vec![20, 50];
    let max_iterations = vec![10_000, 250_000];
    let epsilons = vec![1e-3, 1e-6, 1e-9];

    for function in functions.iter() {
        let name = function.name();
        let mut group = c.benchmark_group(name);
        group.warm_up_time(std::time::Duration::from_millis(250));
        group.nresamples(50);
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.add_row(row![name, "accuracy", "precision", "iteration_nr"]);
        let now = chrono::Local::now();
        let time_str = now.format("%Y%m%d_%H%M%S").to_string();
        for colony_nr in colony_nrs.iter() {
            for colony_size in colony_sizes.iter() {
                for max_iteration in max_iterations.iter() {
                    for epsilon in epsilons.iter() {
                        let mut total = 0;
                        let mut correct = 0;
                        let mut acc_error = 0.0;
                        let mut total_iterations = 0;
                        let min = function.minimum();
                        let name = format!("{}_{}_{}_{}", colony_nr, colony_size, max_iteration, epsilon); 

                        group.bench_function(name.clone(), |b| {
                            b.iter(|| {
                                total += 1;
                                let (_, val, iter_count) = find_minimum(*function, *colony_nr, *colony_size, *max_iteration, false, *epsilon);
                                total_iterations += iter_count;
                                let error = calculate_error(val, min);
                                if error <= *epsilon{
                                    correct += 1;
                                }
                                acc_error += error;
                            });
                        });

                        let accuracy = (correct as f64) / (total as f64);
                        let precision = acc_error / (total as f64);
                        let iteration_count = total_iterations/ total;
                        table.add_row(row![name, accuracy, precision, iteration_count]);
                    }
                }
            }
        }
        let out_csv = File::create(format!("target/benchlogs/{name}_{time_str}.csv")).unwrap();
        table.to_csv(out_csv).unwrap();
        fs::write(format!("target/benchlogs/{name}_{time_str}.txt"), table.to_string()).unwrap();
        group.finish();
    }
}


criterion_group!(benches, test_grid);
criterion_main!(benches);