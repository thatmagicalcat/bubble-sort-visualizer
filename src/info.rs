use std::cmp::Ordering;

use rand::thread_rng;
use rand::Rng;

#[derive(Default)]
pub struct Data {
    ascending: bool,
    list: Vec<f32>,
    i: usize,
    swapped: bool,
}

impl Data {
    pub fn with_size(cap: usize, ascending: bool) -> Self {
        Self {
            ascending,
            list: {
                let mut v = vec![];
                v.resize(cap, Default::default());
                v
            },

            ..Default::default()
        }
    }

    pub fn generate_random(&mut self, min: f32, max: f32) {
        self.list
            .iter_mut()
            .for_each(|i| *i = thread_rng().gen_range(min..=max));
    }

    pub fn get(&self, idx: usize) -> Option<f32> {
        self.list.get(idx).copied()
    }

    pub fn sort_step(&mut self) {
        let n = self.list.len();
        if self.i < n - 1 {
            self.swapped = false;
            for j in 0..n - self.i - 1 {
                if self.list[j].partial_cmp(&self.list[j + 1]).unwrap()
                    == if self.ascending {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                {
                    (self.list[j], self.list[j + 1]) = (self.list[j + 1], self.list[j]);
                    self.swapped = true;
                }
            }

            if !self.swapped {
                self.i = n; // stop
            }

            self.i += 1;
        }
    }

    pub fn is_sorted(&self) -> bool {
        self.i == self.list.len()
    }
}
