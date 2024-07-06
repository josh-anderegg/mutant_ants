use crate::functions::{Function, Point};
use super::genes::Genes;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use super::MAX_AGE;
pub struct Worker {
    pub id: usize,
    pub position: Point,
    genes: Genes,
    function: &'static dyn Function,
    pub value: f64,
    pub remaining_age: usize,
    pub status: Status
}
#[derive(Clone)]
pub enum Action {
    Born(Point),
    Stall(Point),
    Move(Point, Point),
    Die(Point),
    Starve,
    Reproduce(usize)
}
#[derive(PartialEq)]
pub enum Status {
    Starving,
    Trailing,
    Leading
}

impl Worker {
    pub fn iterate(&mut self, gossip: &(Point, f64)) -> (Action, f64) {
        let prev_position = self.position;
        let next_position = self.get_next(gossip);
        let action = if self.function.domain_check(next_position) {
            let val = self.function.eval(next_position).unwrap();
            self.value = val;
            self.position = next_position;
            Action::Move(prev_position, next_position)
        } else {
            Action::Stall(self.position)
        };

        (action, self.value)
    }

    fn get_next(&self, gossip: &(Point, f64)) -> Point {
        let mut rng = thread_rng();

        let spray = self.genes.indecisiveness;
        let spray_x = rng.gen_range(-spray..spray);
        let spray_y = rng.gen_range(-spray..spray);

        let jealousy_rand = rng.gen_range(0.0..1.0);

        let gradient = self.function.gradient(self.position).unwrap_or((0.0, 0.0));
        let next = if self.status == Status::Leading {
            (self.position.0 + (spray_x - gradient.0) * self.genes.stride.0, 
             self.position.1 + (spray_y - gradient.1) * self.genes.stride.0) 
        } else if self.status == Status::Trailing && jealousy_rand < self.genes.jealousy { // Trailing an jealous
            (gossip.0.0 + spray_x, gossip.0.1 + spray_y)
        } else if self.status == Status:: Trailing { // Trailing and not jealous
            let norm = (self.function.norm())/100.0;
            let random_step = (rng.gen_range(-norm..norm), rng.gen_range(-norm..norm));
            (self.position.0 + (spray_x + random_step.0) * self.genes.stride.1, 
             self.position.1 + (spray_y + random_step.1) * self.genes.stride.1)          
        } else {
            let norm = (self.function.norm())/100.0;
            let random_step = (rng.gen_range(-norm..norm), rng.gen_range(-norm..norm));
            (self.position.0 + (spray_x + random_step.0) * self.genes.stride.2, 
             self.position.1 + (spray_x + random_step.1) * self.genes.stride.2) 
        };
        next
    }

    pub fn new(id: usize, colony_center: Point, rng: &mut ThreadRng, function: &'static dyn Function, colony_gene_pool: &Genes) -> Worker {
        let spray = colony_gene_pool.indecisiveness;
        let x_spray = rng.gen_range(-spray..=spray);
        let y_spray = rng.gen_range(-spray..=spray);
        let mut start_position = (colony_center.0 + x_spray, colony_center.1 +  y_spray);
        
        // If the point is outside of the bounds from the start retry generation
        while !function.domain_check(start_position) {
            let x_spray = rng.gen_range(-spray..=spray);
            let y_spray = rng.gen_range(-spray..=spray);
            start_position = (colony_center.0 + x_spray, colony_center.1 +  y_spray);
        }

        let start_val = match function.eval(start_position) {
            Some(val) => val,
            None => std::f64::INFINITY,
        };

        let starting_age = rng.gen_range(1..MAX_AGE);
        Worker {
            id,
            position: start_position,
            genes: colony_gene_pool.mutate(rng),
            function,
            value: start_val,
            remaining_age : starting_age,
            status: Status::Trailing
        }
    }

    pub fn reproduce(&self, id : usize, rng : &mut ThreadRng) -> Worker {
        Worker::new(id,self.position, rng, self.function, &self.genes)
    }
}


