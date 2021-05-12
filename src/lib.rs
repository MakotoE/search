#![feature(drain_filter)]

use std::hash::Hash;

trait Set<V>
where
    V: Hash,
{
    fn contains(&self, item: &V) -> bool;
    fn len(&self) -> usize;
    fn insert(&mut self, item: V);
    fn remove(&mut self, item: &V);
}

#[derive(Default)]
pub struct SimpleArray {
    arr: Vec<i32>,
}

impl Set<i32> for SimpleArray {
    fn contains(&self, item: &i32) -> bool {
        self.arr.iter().any(|i| i == item)
    }

    fn len(&self) -> usize {
        self.arr.len()
    }

    fn insert(&mut self, item: i32) {
        self.arr.push(item);
    }

    fn remove(&mut self, item: &i32) {
        self.arr.drain_filter(|i| i == item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    macro_rules! set {
        ( $set_type:ty ) => {
            (|| Box::new(<$set_type>::default()), stringify!($set_type))
        };
    }

    lazy_static! {
        static ref SETS: [(fn() -> Box<dyn Set<i32>>, &'static str); 1] = [set!(SimpleArray)];
    }

    #[test]
    fn test() {
        for (constructor, name) in SETS.iter() {
            let mut set = constructor();
            assert!(!set.contains(&0), "{}", name);
            set.insert(0);
            set.insert(2);
            set.insert(1);
            assert!(set.contains(&0), "{}", name);
            assert!(set.contains(&1), "{}", name);
            assert!(set.contains(&2), "{}", name);
            set.remove(&0);
            assert!(!set.contains(&0), "{}", name);
            assert!(set.contains(&1), "{}", name);
            assert!(set.contains(&2), "{}", name);
            set.remove(&1);
            set.remove(&2);
            assert_eq!(set.len(), 0);
        }
    }
}
