extern crate test;

use self::test::Bencher;
use crate::{OrderedArray, Set, SimpleArray};
use rand::distributions::Standard;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

fn items() -> [i32; 1024] {
    let mut array = [0; 1024];
    for (i, n) in SmallRng::seed_from_u64(69)
        .sample_iter(Standard)
        .take(1024)
        .enumerate()
    {
        array[i] = n
    }
    array
}

#[bench]
fn simple_array(b: &mut Bencher) {
    let items = items();
    b.iter(|| {
        let mut set = SimpleArray::default();
        for item in items.iter().copied() {
            set.insert(item);
        }
        assert!(items.contains(&-1431199289));
    });
}

#[bench]
fn ordered_array(b: &mut Bencher) {
    let items = items();
    b.iter(|| {
        let mut set = OrderedArray::default();
        for item in items.iter().copied() {
            set.insert(item);
        }
        assert!(items.contains(&-1431199289));
    });
}
