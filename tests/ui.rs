#[test]
fn ui() {
    if version_check::is_min_version("1.84.0").unwrap() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/*.rs");
    }
}
