#[test]
fn ui() {
    if std::env::var("TOOLCHAIN").as_deref() != Ok("nightly") {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/*.rs");
    }
}
