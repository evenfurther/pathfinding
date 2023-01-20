use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use pathfinding::directed::edmonds_karp::*;
use std::collections::HashMap;

/// Return a list of edges with their capacities.
fn successors_wikipedia() -> Vec<((char, char), i32)> {
    vec![
        ("AB", 3),
        ("AD", 3),
        ("BC", 4),
        ("CA", 3),
        ("CD", 1),
        ("CE", 2),
        ("DE", 2),
        ("DF", 6),
        ("EB", 1),
        ("EG", 1),
        ("FG", 9),
    ]
    .into_iter()
    .map(|(s, c)| {
        let mut name = s.chars();
        ((name.next().unwrap(), name.next().unwrap()), c)
    })
    .collect()
}

fn check_wikipedia_result(flows: EKFlows<char, i32>) {
    let (caps, total, _cuts) = flows;
    assert_eq!(caps.len(), 8);
    let caps = caps.into_iter().collect::<HashMap<(char, char), i32>>();
    assert_eq!(caps[&('A', 'B')], 2);
    assert_eq!(caps[&('A', 'D')], 3);
    assert_eq!(caps[&('B', 'C')], 2);
    assert_eq!(caps[&('C', 'D')], 1);
    assert_eq!(caps[&('C', 'E')], 1);
    assert_eq!(caps[&('D', 'F')], 4);
    assert_eq!(caps[&('E', 'G')], 1);
    assert_eq!(caps[&('F', 'G')], 4);
    assert_eq!(total, 5);
}

fn wikipedia_example<EK: EdmondsKarp<i32>>(c: &mut Criterion, id: &str) {
    c.bench_function(id, |b| {
        b.iter(|| {
            check_wikipedia_result(edmonds_karp::<_, _, _, EK>(
                &"ABCDEFGH".chars().collect::<Vec<_>>(),
                &'A',
                &'G',
                successors_wikipedia(),
            ))
        })
    });
}

fn wikipedia_example_dense(c: &mut Criterion) {
    wikipedia_example::<DenseCapacity<_>>(c, "wikipedia_example_dense");
}

fn wikipedia_example_sparse(c: &mut Criterion) {
    wikipedia_example::<SparseCapacity<_>>(c, "wikipedia_example_sparse");
}

criterion_group!(benches, wikipedia_example_dense, wikipedia_example_sparse,);
criterion_main!(benches);
