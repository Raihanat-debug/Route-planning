use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_sample() {
    let input = b"5 6
1 2 4
1 3 2
2 4 5
3 4 1
4 5 3
3 5 8
1 5
";

    let mut child = Command::new("cargo")
        .args(["run", "--bin", "astar"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run astar");

    child.stdin.as_mut().unwrap().write_all(input).unwrap();

    let output = child.wait_with_output().unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.trim(), "6");
}

#[test]
fn test_direct_path() {
    let input = b"2 1
1 2 5
1 2
";

    let mut child = Command::new("cargo")
        .args(["run", "--bin", "astar"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run astar");

    child.stdin.as_mut().unwrap().write_all(input).unwrap();

    let output = child.wait_with_output().unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.trim(), "5");
}

#[test]
fn test_no_path() {
    let input = b"4 2
1 2 3
3 4 4
1 4
";

    let mut child = Command::new("cargo")
        .args(["run", "--bin", "astar"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run astar");

    child.stdin.as_mut().unwrap().write_all(input).unwrap();

    let output = child.wait_with_output().unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.trim(), "NO PATH");
}