use std::collections::btree_map::Range;
use rand::{rngs::ThreadRng, Rng};

pub struct Sim {
    n_list: Vec<i32>,
    exec_pos: i16,
    rng: ThreadRng,
}
impl Sim {
    pub fn new(n: i32) -> Sim {
        let n_list = (1..=n).collect();
        let exec_pos = -1;
        let rng = rand::thread_rng();
        return Sim{
            n_list: n_list,
            exec_pos: exec_pos,
            rng: rng
        };
    }
    pub fn run(&mut self) -> i32 {
        while (self.n_list.len() > 1) {
            self.step();
        }
        return self.n_list[0];
    }
    pub fn step(&mut self) {
        let r = self.rng.gen_range(1..3);
        self.exec_pos = (self.exec_pos+r)%(self.n_list.len() as i16);
        self.n_list.remove(self.exec_pos as usize);
        self.exec_pos -= 1;
    }

}