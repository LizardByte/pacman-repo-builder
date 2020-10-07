use pipe_trait::*;
use std::{path::PathBuf, process::Command};

const EXE: &str = env!("CARGO_BIN_EXE_build-pacman-repo");
const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn work_dir() -> PathBuf {
    ROOT.pipe(PathBuf::from)
        .join("tests")
        .join("fixtures")
        .join("print-config")
}

fn init() -> Command {
    let mut command = Command::new(EXE);
    command
        .current_dir(work_dir())
        .arg("print-config")
        .args(&["--container", "mixed"])
        .args(&["--container", "pkgbuild-only"])
        .args(&["--container", "srcinfo-only"])
        .args(&["--container", "pkgbuild-and-srcinfo"]);
    command
}

fn output(command: &mut Command) -> (String, String, bool) {
    let output = command.output().expect("get output from a command");
    let stdout = output
        .stdout
        .pipe(String::from_utf8)
        .expect("convert stdout to UTF-8");
    let stderr = output
        .stderr
        .pipe(String::from_utf8)
        .expect("convert stderr to UTF-8");
    let success = output.status.success();
    (stdout, stderr, success)
}

#[test]
fn require_nothing() {
    let (stdout, stderr, success) = output(&mut init());
    let actual = (stdout.trim(), stderr.trim(), success);
    let expected = (
        include_str!("./expected-output/print-config/require-nothing.stdout.yaml").trim(),
        "",
        true,
    );
    assert_eq!(actual, expected);
}

#[test]
fn require_pkgbuild() {
    let (stdout, stderr, success) = init().arg("--require-pkgbuild").pipe(output);
    let actual = (stdout.trim(), stderr.trim(), success);
    let expected = (
        include_str!("./expected-output/print-config/require-pkgbuild.stdout.yaml").trim(),
        "",
        true,
    );
    assert_eq!(actual, expected);
}

#[test]
fn require_srcinfo() {
    let (stdout, stderr, success) = init().arg("--require-srcinfo").pipe(output);
    let actual = (stdout.trim(), stderr.trim(), success);
    let expected = (
        include_str!("./expected-output/print-config/require-srcinfo.stdout.yaml").trim(),
        "",
        true,
    );
    assert_eq!(actual, expected);
}

#[test]
fn require_pkgbuild_and_srcinfo() {
    let (stdout, stderr, success) = init()
        .arg("--require-pkgbuild")
        .arg("--require-srcinfo")
        .pipe(output);
    let actual = (stdout.trim(), stderr.trim(), success);
    let expected = (
        include_str!("./expected-output/print-config/require-pkgbuild-and-srcinfo.stdout.yaml")
            .trim(),
        "",
        true,
    );
    assert_eq!(actual, expected);
}

#[test]
fn with_one_repository() {
    let (stdout, stderr, success) = init().args(&["--repository", "repository"]).pipe(output);
    let actual = (stdout.trim(), stderr.trim(), success);
    let expected = (
        include_str!("./expected-output/print-config/with-one-repository.stdout.yaml").trim(),
        "",
        true,
    );
    assert_eq!(actual, expected);
}

#[test]
fn with_multiple_repositories() {
    let (stdout, stderr, success) = init()
        .args(&["--repository", "foo"])
        .args(&["--repository", "bar"])
        .args(&["--repository", "baz"])
        .pipe(output);
    let actual = (stdout.trim(), stderr.trim(), success);
    let expected = (
        include_str!("./expected-output/print-config/with-multiple-repositories.stdout.yaml")
            .trim(),
        "",
        true,
    );
    assert_eq!(actual, expected);
}