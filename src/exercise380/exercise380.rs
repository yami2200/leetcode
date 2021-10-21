use std::collections::HashSet;
use std::iter::FromIterator;
use rand::prelude::*;

pub(crate) struct RandomizedSet {
    storage: HashSet<i32>,
}


impl RandomizedSet {

    pub(crate) fn new() -> Self {
        RandomizedSet{
            storage: HashSet::new(),
        }
    }

    pub(crate) fn insert(&mut self, val: i32) -> bool {
        self.storage.insert(val)
    }

    pub(crate) fn remove(&mut self, val: i32) -> bool {
        self.storage.remove(&val)
    }

    pub(crate) fn get_random(&self) -> i32 {
        let v: Vec<&i32> = Vec::from_iter(&self.storage);
        let mut rng = rand::thread_rng();
        *v[rng.gen_range(0..v.len())]
    }
}