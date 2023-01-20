// Test with files from https://movingai.com/benchmarks/

use codspeed_criterion_compat::*;
use movingai::parser::{parse_map_file, parse_scen_file};
use movingai::{Coords2D, Map2D};
use noisy_float::prelude::*;
use pathfinding::directed::astar::astar;
use std::path::Path;

fn distance(a: &Coords2D, b: &Coords2D) -> R64 {
    r64((a.0 as f64 - b.0 as f64).hypot(a.1 as f64 - b.1 as f64))
}

pub fn arena(c: &mut Criterion) {
    c.bench_function("arena", |b| {
        b.iter(|| {
            let map = parse_map_file(Path::new("./benches/arena.map")).unwrap();
            let scenes = parse_scen_file(Path::new("./benches/arena.map.scen")).unwrap();
            for scene in scenes {
                let start = scene.start_pos;
                let goal = scene.goal_pos;
                let result = astar(
                    &start,
                    |&node| {
                        map.neighbors(node)
                            .into_iter()
                            .map(move |n| (n, distance(&node, &n)))
                    },
                    |&node| distance(&node, &goal),
                    |&node| node == goal,
                )
                .unwrap();
                assert!(result.1 - r64(scene.optimal_length).abs() <= 1e-4);
            }
        })
    });
}

criterion_group!(benches, arena);
criterion_main!(benches);
