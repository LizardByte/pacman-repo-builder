use pacman_repo_builder::{status::Code, utils::PackageFileName};
use std::{
    env,
    fs::{self, Permissions},
    iter::once,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::Command,
};
use tempfile::TempDir;

const EXE: &str = env!("CARGO_BIN_EXE_build-pacman-repo");
const PACKAGE: &str = "example-1-1-x86_64.pkg.tar.zst";
const RECORD: &str = "failed-builds.yaml";

struct Context {
    work_dir: TempDir,
}

impl Context {
    fn new(settings: &str) -> Self {
        let context = Self {
            work_dir: TempDir::new().expect("create build fixture"),
        };
        for directory in ["bin", "source", "repo"] {
            fs::create_dir(context.path(directory)).expect("create fixture directory");
        }
        fs::write(
            context.path("build-pacman-repo.yaml"),
            format!(
                "global-settings:\n  repository: repo/test.db.tar.gz\n  read-build-metadata: srcinfo\n\
                 {settings}\nmembers:\n  - directory: source\n"
            ),
        )
        .expect("write build manifest");
        fs::write(
            context.path("source/.SRCINFO"),
            "pkgbase = example\n\tpkgver = 1\n\tpkgrel = 1\n\tarch = x86_64\n\npkgname = example\n",
        )
        .expect("write package metadata");

        context.tool(
            "makepkg",
            "printf 'package contents' > example-1-1-\"$CARCH\".pkg.tar.zst",
        );
        context.tool("pacman", "exit 0");
        context.tool(
            "pacman-conf",
            "printf '[options]\\nArchitecture = x86_64\\n'",
        );
        context.tool("repo-add", "exit 0");
        context
    }

    fn path(&self, path: &str) -> PathBuf {
        self.work_dir.path().join(path)
    }

    fn tool(&self, name: &str, body: &str) {
        let script = self.path("bin").join(name);
        fs::write(
            &script,
            format!("#!/bin/sh\nset -eu\nprintf '%s\\n' \"$@\" >> \"$TEST_BUILD_DIR/{name}.args\"\n{body}\n"),
        )
        .expect("write fake command");
        fs::set_permissions(script, Permissions::from_mode(0o755))
            .expect("make command executable");
    }

    fn read(&self, path: &str) -> String {
        fs::read_to_string(self.path(path)).expect("read build output")
    }

    fn build(&self, expected_status: i32) -> String {
        let inherited_path = env::var_os("PATH").unwrap_or_default();
        let path = env::join_paths(once(self.path("bin")).chain(env::split_paths(&inherited_path)))
            .expect("set search path for fake commands");
        let output = Command::new(EXE)
            .arg("build")
            .current_dir(self.work_dir.path())
            .env("PATH", path)
            .env("TEST_BUILD_DIR", self.work_dir.path())
            .output()
            .expect("run build");
        let stderr = String::from_utf8(output.stderr).expect("read stderr");
        assert_eq!(output.status.code(), Some(expected_status), "{stderr}");
        stderr
    }
}

#[test]
fn publishes_package_after_dependency_install_warning() {
    let context = Context::new("  install-missing-dependencies: true\n  clean-after-build: true");
    let srcinfo = context.read("source/.SRCINFO");
    fs::write(
        context.path("source/.SRCINFO"),
        srcinfo.replace(
            "\n\npkgname",
            "\n\tdepends = prb-test-missing-dependency\n\npkgname",
        ),
    )
    .expect("add missing dependency to package metadata");
    context.tool("pacman", "exit 17");

    let stderr = context.build(0);

    assert!(
        stderr.contains("pacman -S exits with non-zero status code: 17"),
        "{stderr}"
    );
    assert_eq!(
        context.read("pacman.args"),
        "--sync\nprb-test-missing-dependency\n--noconfirm\n--asdeps\n--needed\n"
    );
    assert_eq!(
        context.read("makepkg.args"),
        "--install\n--noconfirm\n--asdeps\n--syncdeps\n--clean\n"
    );
    assert_eq!(context.read(&format!("repo/{PACKAGE}")), "package contents");
    assert!(!context.path(&format!("source/{PACKAGE}")).exists());
    assert_eq!(
        context.read("repo-add.args"),
        format!("--quiet\n--nocolor\nrepo/test.db.tar.gz\nrepo/{PACKAGE}\n")
    );
}

#[test]
fn records_allowed_failure_and_skips_it_on_the_next_build() {
    let context = Context::new("  allow-failure: true\n  record-failed-builds: failed-builds.yaml");
    let previous = "- pkgname: other\n  version: 2-1\n  arch: any\n";
    fs::write(context.path(RECORD), previous).expect("seed failure record");
    context.tool("makepkg", "exit 19");

    let stderr = context.build(0);

    assert!(stderr.contains("Some builds failed:"), "{stderr}");
    assert!(!context.path("repo-add.args").exists());
    let parse_record = |yaml: &str| {
        serde_yaml::from_str::<Vec<PackageFileName<String, String, String>>>(yaml)
            .expect("parse failure record")
    };
    let record = context.read(RECORD);
    assert_eq!(
        parse_record(&record),
        parse_record(&format!(
            "{previous}- pkgname: example\n  version: 1-1\n  arch: x86_64\n"
        ))
    );

    fs::remove_file(context.path("makepkg.args")).expect("clear makepkg log");
    let stderr = context.build(0);
    assert!(
        stderr.contains("Failures had been recorded. Skip."),
        "{stderr}"
    );
    assert!(!context.path("makepkg.args").exists());
    assert_eq!(context.read(RECORD), record);
}

#[test]
fn returns_makepkg_failure_when_it_is_disallowed() {
    let context = Context::new("  record-failed-builds: failed-builds.yaml");
    context.tool("makepkg", "exit 19");

    let stderr = context.build(19);

    assert!(
        stderr.contains("makepkg exits with non-zero status code: 19"),
        "{stderr}"
    );
    assert!(!context.path("repo-add.args").exists());
    assert!(!context.path(RECORD).exists());
}

#[test]
fn returns_repo_add_failure_after_copying_the_package() {
    let context = Context::new("");
    context.tool("repo-add", "exit 23");

    let stderr = context.build(23);

    assert!(
        stderr.contains("repo-add exits with non-zero status code: 23"),
        "{stderr}"
    );
    assert_eq!(context.read(&format!("repo/{PACKAGE}")), "package contents");
    assert!(context.path(&format!("source/{PACKAGE}")).is_file());
}

#[test]
fn returns_an_error_if_the_failed_build_record_cannot_be_written() {
    let context =
        Context::new("  allow-failure: true\n  record-failed-builds: missing/record.yaml");
    context.tool("makepkg", "exit 19");

    let stderr = context.build(Code::FailedBuildRecordWritingFailure as i32);

    assert!(stderr.contains("Some builds failed:"), "{stderr}");
    assert!(!context.path("missing/record.yaml").exists());
}
