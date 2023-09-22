#[test]
fn ui() {
    match std::env::var("TOOLCHAIN").as_deref() {
        Ok("stable") | Ok("1.70.0") => (),
        _ => {
            let t = trybuild::TestCases::new();
            t.compile_fail("tests/ui/*.rs");
        }
    }
}
