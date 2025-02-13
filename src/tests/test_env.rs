use crate::tests::{run_test, TestResult};

#[test]
fn shorthand_env_1() -> TestResult {
    run_test(r#"FOO=BAZ $nu.env.FOO"#, "BAZ")
}

#[test]
fn shorthand_env_2() -> TestResult {
    run_test(r#"FOO=BAZ FOO=MOO $nu.env.FOO"#, "MOO")
}

#[test]
fn shorthand_env_3() -> TestResult {
    run_test(r#"FOO=BAZ BAR=MOO $nu.env.FOO"#, "BAZ")
}
