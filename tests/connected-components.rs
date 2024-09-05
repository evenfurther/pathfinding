use itertools::Itertools;
use pathfinding::undirected::connected_components::*;
use rand::prelude::*;
use rand_xorshift::XorShiftRng;
use std::collections::HashSet;

#[test]
fn basic_separate_components() {
    let groups = [vec![1, 2], vec![3, 4], vec![5, 6], vec![1, 4]];
    let (h, g) = separate_components(&groups);
    assert!([1, 2, 3, 4].iter().map(|n| h[n]).all_equal());
    assert_eq!(h[&5], h[&6]);
    assert!(h[&1] != h[&5]);
    assert_eq!(h.len(), 6);
    assert_eq!(g[0], g[1]);
    assert_eq!(g[0], g[3]);
    assert!(g[0] != g[2]);
    assert_eq!(g.len(), 4);
}

#[test]
fn empty_separate_components() {
    let groups = [vec![1, 2], vec![3, 4], vec![], vec![1, 4]];
    let (h, g) = separate_components(&groups);
    assert!([1, 2, 3, 4].iter().map(|n| h[n]).all_equal());
    assert_eq!(h.len(), 4);
    assert_eq!(g[0], g[1]);
    assert_eq!(g[0], g[3]);
    assert!(g[0] != g[2]);
    assert_eq!(g[2], usize::MAX);
    assert_eq!(g.len(), 4);
}

#[test]
fn basic_components() {
    let mut c = components(&[vec![1, 2], vec![3, 4], vec![5, 6], vec![1, 4, 7]]);
    c.sort_unstable_by_key(|v| *v.iter().min().unwrap());
    assert_eq!(c.len(), 2);
    assert_eq!(
        c[0].clone().into_iter().sorted().collect_vec(),
        vec![1, 2, 3, 4, 7]
    );
    assert_eq!(c[1].clone().into_iter().sorted().collect_vec(), vec![5, 6]);
}

#[test]
fn empty_components() {
    let mut c = components(&[vec![1, 2], vec![3, 4], vec![], vec![1, 4, 7]]);
    c.sort_unstable_by_key(|v| *v.iter().min().unwrap());
    assert_eq!(c.len(), 1);
    assert_eq!(
        c[0].clone().into_iter().sorted().collect_vec(),
        vec![1, 2, 3, 4, 7]
    );
}

#[test]
fn basic_connected_components() {
    let mut counter = 0;
    let mut c = connected_components(&[1, 4], |&n| {
        counter += 1;
        if n % 2 == 0 {
            vec![2, 4, 6, 8]
        } else {
            vec![1, 3, 5, 7]
        }
    });
    c.sort_unstable_by_key(|v| *v.iter().min().unwrap());
    assert_eq!(c.len(), 2);
    assert_eq!(
        c[0].clone().into_iter().sorted().collect_vec(),
        vec![1, 3, 5, 7]
    );
    assert_eq!(
        c[1].clone().into_iter().sorted().collect_vec(),
        vec![2, 4, 6, 8]
    );
    assert_eq!(counter, 2);
}

#[test]
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
    components.sort_unstable_by_key(|v| *v.iter().min().unwrap());
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
