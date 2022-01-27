#![cfg(feature = "trybuild_tests")]
#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
    t.pass("tests/pass/*.rs");
}
