const VERSION: &str = env!("CARGO_PKG_VERSION");
const README: &str = include_str!("../README.md");

#[test]
fn check_version() {
    let line = README
        .lines()
        .find(|l| l.starts_with("pathfinding = "))
        .expect("no version line found in README.md");
    let version = line.split('"').collect::<Vec<_>>()[1];
    assert!(
        VERSION.starts_with(version),
        format!(
            "Version in README.md ({}) is not compatible with Cargo.toml ({})",
            version,
            VERSION
        )
    );
}
