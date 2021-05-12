#![feature(drain_filter)]

use std::fmt::Debug;
use std::hash::Hash;

trait Set<T>: Debug
where
    T: Hash,
{
    fn contains(&self, item: &T) -> bool;
    fn len(&self) -> usize;
    fn insert(&mut self, item: T);
    fn remove(&mut self, item: &T);
}

#[derive(Debug, Default)]
pub struct SimpleArray<T> {
    arr: Vec<T>,
}

impl<T: Hash + Debug + PartialEq> Set<T> for SimpleArray<T> {
    fn contains(&self, item: &T) -> bool {
        self.arr.iter().any(|i| i == item)
    }

    fn len(&self) -> usize {
        self.arr.len()
    }

    fn insert(&mut self, item: T) {
        if !self.contains(&item) {
            self.arr.push(item);
        }
    }

    fn remove(&mut self, item: &T) {
        self.arr.drain_filter(|i| i == item);
    }
}

#[derive(Debug, Default)]
pub struct OrderedArray<T> {
    arr: Vec<T>,
}

impl<T: Hash + Debug + PartialEq + PartialOrd> Set<T> for OrderedArray<T> {
    fn contains(&self, item: &T) -> bool {
        if self.arr.len() == 0 {
            return false;
        }

        let mut left = 0_usize;
        let mut right = self.arr.len() - 1;
        while left <= right {
            let middle = (left + right) / 2;
            if &self.arr[middle] < item {
                left = middle + 1;
            } else if &self.arr[middle] > item {
                if middle == 0 {
                    break;
                }
                right = middle - 1;
            } else {
                return true;
            }
        }

        return false;
    }

    fn len(&self) -> usize {
        self.arr.len()
    }

    fn insert(&mut self, item: T) {
        if self.arr.len() == 0 {
            self.arr.insert(0, item);
            return;
        }

        let mut left = 0_usize;
        let mut right = self.arr.len() - 1;
        while left <= right {
            let middle = (left + right) / 2;
            if self.arr[middle] < item {
                left = middle + 1;
            } else if self.arr[middle] > item {
                if middle == 0 {
                    break;
                }
                right = middle - 1;
            } else {
                return;
            }
        }

        self.arr.insert(left, item);
    }

    fn remove(&mut self, item: &T) {
        if self.arr.len() == 0 {
            return;
        }

        let mut left = 0_usize;
        let mut right = self.arr.len() - 1;
        while left <= right {
            let middle = (left + right) / 2;
            if &self.arr[middle] < item {
                left = middle + 1;
            } else if &self.arr[middle] > item {
                if middle == 0 {
                    break;
                }
                right = middle - 1;
            } else {
                self.arr.remove(middle);
                return;
            }
        }
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
        static ref SETS: [(fn() -> Box<dyn Set<i32>>, &'static str); 2] =
            [set!(SimpleArray<i32>), set!(OrderedArray<i32>)];
    }

    #[test]
    fn test() {
        for (constructor, name) in SETS.iter() {
            let mut set = constructor();
            assert!(!set.contains(&0), "{}", name);
            set.insert(0);
            set.insert(0);
            assert_eq!(set.len(), 1, "{}", name);
            set.insert(2);
            set.insert(1);
            assert_eq!(set.len(), 3, "{}", name);
            assert!(set.contains(&0), "{}", name);
            assert!(set.contains(&1), "{}", name);
            assert!(set.contains(&2), "{}", name);
            set.remove(&0);
            assert!(!set.contains(&0), "{}", name);
            assert!(set.contains(&1), "{}", name);
            assert!(set.contains(&2), "{}", name);
            set.remove(&2);
            set.remove(&1);
            assert_eq!(set.len(), 0);
            set.remove(&0);
        }
    }
}
