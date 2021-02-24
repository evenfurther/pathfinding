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
        trim(VERSION).starts_with(&trim(version)),
        "Version in README.md ({} - seen as {}) is not compatible with Cargo.toml ({} - seen as {})",
        version, trim(version), VERSION, trim(VERSION),
    );
}

// Keep at most two components (major/minor).
fn trim(version: &str) -> String {
    version.split('.').take(2).collect::<Vec<_>>().join(".")
}
