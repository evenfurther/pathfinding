use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use pathfinding::prelude::separate_components;
use rand::{prelude::SliceRandom, Rng as _, RngCore as _, SeedableRng as _};
use rand_xorshift::XorShiftRng;
use std::collections::HashSet;

fn larger_separate_components() {
    // Create 100 groups of 100 elements, then randomly split
    // into sub-groups.
    let mut rng = XorShiftRng::from_seed([
        3, 42, 93, 129, 1, 85, 72, 42, 84, 23, 95, 212, 253, 10, 4, 2,
    ]);
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
                let cut = rng.gen_range(0..component.len());
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

fn bench_separate_components(c: &mut Criterion) {
    c.bench_function("separate_components", |b| {
        b.iter(larger_separate_components);
    });
}

criterion_group!(benches, bench_separate_components);
criterion_main!(benches);
