# Description

Library for finding minimas of arbitrary functions written in Rust. 

## Details

This library uses the following paradigms:
 - [Particle swarm optimization](https://en.wikipedia.org/wiki/Particle_swarm_optimization)
 - [Genetic algorithm](https://en.wikipedia.org/wiki/Genetic_algorithm)
 - [Stochastic Gradient Descent](https://en.wikipedia.org/wiki/Stochastic_gradient_descent)
 - Parallelism with Rust

The general optimization approach involves multiple processes (ant colonies) exploring the function space. Each colony possesses workers (ants) that move in the function space in each iteration. The ants move according to their position in the hierarchy, ordered from the worker with the lowest (best) function value to the worker with the highest (worst) function value. Generally, the worse off the worker, the more desperate and sporadic their movement. This ensures that some workers might escape local minimas, while better-off workers tend to simply descend the local minima to the best position. The exact parameters for movement depend on the genes of the worker, which are passed on to offspring. Offspring is only produced if the worker is in a good position, leading to natural selection. At the end of an optimization run, the position of the very best worker of all colonies is returned. The underlying assumption of the function space is that all values are non-negative to match the description of a [loss function](https://en.wikipedia.org/wiki/Loss_function), therefore early stoppage is possible if a certain epsilon precision from zero is achieved by the workers.

# Usage

## Defining your own functions
The library provides a Rust trait that allows you to define your own function to optimize. See [examples](src/functions) and [trait implementation](src/function.rs). If the minimum value does not match $0$, the library will still find a minima, but as of now will not return early.

## Drawing a function run
This package provides a way to draw an execution of an optimization to a gif. For this, run the [draw_all.sh](draw_all.sh) bash script. This requires ffmpeg to be installed and if you want your function to be drawn, you have to edit [this file](tests/draw_tests.rs).

## Running benchmarks
This library allows running benchmarks by executing [run_benches.sh](run_benches.sh). The benchmarks provide results on the general precision, accuracy, and iteration numbers in the target directory. For the benchmarks, we used [criterion](https://docs.rs/criterion/latest/criterion/), which will output a standard criterion benchmark result as well. Caution, the benchmarks run very slowly; a standard run can be seen in [results](results).