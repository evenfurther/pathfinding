#[test]
fn ui() {
    match std::env::var("TOOLCHAIN").as_deref() {
        Ok("1.65.0") => (),
        _ => {
            let t = trybuild::TestCases::new();
            t.compile_fail("tests/ui/*.rs");
        }
    }
}
