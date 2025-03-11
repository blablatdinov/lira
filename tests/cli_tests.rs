use assert_cmd::Command;

#[test]
fn test_without_arguments() {
    let mut cmd = Command::cargo_bin("onlivi").unwrap();
    cmd.assert()
        .success()
        .stdout("Empty line range\n");
}

#[test]
fn test_with_line_range_argument() {
    let mut cmd = Command::cargo_bin("onlivi").unwrap();
    cmd.arg("--line-range").arg("1-5").assert()
        .success()
        .stdout("Line range: 1-5\n");
}