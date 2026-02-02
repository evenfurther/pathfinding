use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use itertools::Itertools;
use pathfinding::prelude::separate_components;
use rand::{Rng as _, RngCore as _, prelude::SliceRandom};
use rand_core::SeedableRng as _;
use rand_xorshift::XorShiftRng;
use std::collections::HashSet;

// Adapter to make XorShiftRng (which implements rand_core 0.10 traits)
// compatible with rand 0.9.2 (which expects rand_core 0.9 traits)
struct RngAdapter<R>(R);

#[expect(deprecated)]
impl<R: rand_core::RngCore> rand::RngCore for RngAdapter<R> {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest);
    }
}

const RNG_SEED: [u8; 16] = [
    3, 42, 93, 129, 1, 85, 72, 42, 84, 23, 95, 212, 253, 10, 4, 2,
];

fn larger_separate_components() {
    // Create 100 groups of 100 elements, then randomly split
    // into sub-groups.
    let mut rng = RngAdapter(XorShiftRng::from_seed(RNG_SEED));
    let mut seen = HashSet::new();
    let mut components = (0..100)
        .map(|_| {
            let mut component = Vec::new();
            for _ in 0..100 {
                let node = rng.next_u64();
                if !seen.contains(&node) {
                    seen.insert(node);
                    component.push(node);
                }
            }
            component.sort_unstable();
            assert!(
                !component.is_empty(),
                "component is empty, rng seed needs changing"
            );
            component
        })
        .collect_vec();
    components.sort_unstable_by_key(|c| c[0]);
    let mut groups = components
        .iter()
        .flat_map(|component| {
            let mut component = component.clone();
            component.shuffle(&mut rng);
            let mut subcomponents = Vec::new();
            while !component.is_empty() {
                let cut = rng.random_range(0..component.len());
                let mut subcomponent = component.drain(cut..).collect_vec();
                if !component.is_empty() {
                    subcomponent.push(component[0]);
                }
                subcomponent.shuffle(&mut rng);
                subcomponents.push(subcomponent);
            }
            subcomponents
        })
        .collect_vec();
    groups.shuffle(&mut rng);
    let (_, group_mappings) = separate_components(&groups);
    let mut out_groups = vec![HashSet::new(); groups.len()];
    for (i, n) in group_mappings.into_iter().enumerate() {
        assert!(
            n < groups.len(),
            "group index is greater than expected: {}/{}",
            n,
            groups.len()
        );
        for e in &groups[i] {
            out_groups[n].insert(*e);
        }
    }
    let out_groups = out_groups
        .into_iter()
        .map(|g| g.into_iter().collect_vec())
        .collect_vec();
    let mut out_groups = out_groups
        .into_iter()
        .filter_map(|mut group| {
            if group.is_empty() {
                None
            } else {
                group.sort_unstable();
                Some(group)
            }
        })
        .collect_vec();
    out_groups.sort_by_key(|c| c[0]);
    assert_eq!(out_groups, components);
}

#[library_benchmark]
fn bench_separate_components() {
    larger_separate_components();
}

library_benchmark_group!(
    name = separate_components;
    benchmarks = bench_separate_components
);

main!(library_benchmark_groups = separate_components);
