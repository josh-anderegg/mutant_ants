#[cfg(test)]
const COLONY_COUNT: usize  = 10;
const COLONY_SIZE: usize = 20;
const EPSILON: f64 = 1e-6; // Small epsilon onto which we desire accuracy
const MAX_DRAW_ITERATIIONS: usize = 500;

use mutant_ants::{find_minimum, functions::{Ackley, EggContour, GoldsteinPrice, Himmelblau, Levi, Parabolla, Rastrigin, Rosenbrock, Schaffer4}};

#[test]
#[ignore]
fn draw_rastrigin() {
    let _ = find_minimum(&Rastrigin, COLONY_COUNT, COLONY_SIZE, MAX_DRAW_ITERATIIONS,true, EPSILON);
}

#[test]
#[ignore]
fn draw_rosenbrock() {
    let _ = find_minimum(&Rosenbrock, COLONY_COUNT, COLONY_SIZE, MAX_DRAW_ITERATIIONS,true, EPSILON);
}

#[test]
#[ignore]

fn draw_ackley() {
    let _ = find_minimum(&Ackley, 10, 10, MAX_DRAW_ITERATIIONS, true, EPSILON);
}

#[test]
#[ignore]
fn draw_parabolla() {
    let _ = find_minimum(&Parabolla, 10, 10, MAX_DRAW_ITERATIIONS, true, EPSILON);
}

#[test]
#[ignore]
fn draw_schaffer2() {
    let _ = find_minimum(&Schaffer4, 10, 10, MAX_DRAW_ITERATIIONS, true, EPSILON);
}

#[test]
#[ignore]
fn draw_egg_contour() {
    let _ = find_minimum(&EggContour, 10, 10, MAX_DRAW_ITERATIIONS, true, EPSILON);
}

#[test]
#[ignore]
fn draw_golstein_price() {
    let _ = find_minimum(&GoldsteinPrice, 10, 10, MAX_DRAW_ITERATIIONS, true, EPSILON);
}

#[test]
#[ignore]
fn draw_levi() {
    let _ = find_minimum(&Levi, 10, 10, MAX_DRAW_ITERATIIONS, true, EPSILON);
}

#[test]
#[ignore]
fn draw_himmelblau() {
    let _ = find_minimum(&Himmelblau, 10, 10, MAX_DRAW_ITERATIIONS, true, EPSILON);
}