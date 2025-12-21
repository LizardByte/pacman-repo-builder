use command_extra::CommandExtra;
use std::process::Command;

/// Creates a makepkg command that runs as the builder user via sudo.
///
/// In Docker environments, the container runs as root to avoid permission issues
/// with mounted volumes. However, makepkg refuses to run as root for security reasons.
/// Therefore, we run makepkg as the `builder` user using sudo.
///
/// The builder user is created in the Dockerfile.
///
/// # Returns
///
/// A `Command` configured to run `sudo -u builder makepkg` with appropriate
/// environment variables set/unset for clean package builds.
pub fn create_makepkg_command() -> Command {
    Command::new("sudo")
        .with_arg("-u")
        .with_arg("builder")
        .with_arg("makepkg")
        .without_env("PACMAN")
        .without_env("MAKEPKG_CONF")
        .without_env("PKGDEST")
        .without_env("SRCDEST")
        .without_env("LOGDEST")
        .without_env("PACKAGER")
        .without_env("SRCPKGDEST")
        .without_env("BUILDDIR")
        .without_env("GNUPGHOME")
        .without_env("GPGKEY")
        .without_env("SOURCE_DATE_EPOCH")
        .with_env("PKGEXT", ".pkg.tar.zst")
        .with_env("SRCEXT", ".src.tar.gz")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_makepkg_command_uses_sudo() {
        let cmd = create_makepkg_command();
        assert_eq!(cmd.get_program(), "sudo");
    }

    #[test]
    fn test_create_makepkg_command_has_builder_user() {
        let cmd = create_makepkg_command();
        let args: Vec<_> = cmd.get_args().map(|s| s.to_string_lossy()).collect();

        // Should contain -u, builder, makepkg in that order
        assert!(args.len() >= 3);
        assert_eq!(args[0], "-u");
        assert_eq!(args[1], "builder");
        assert_eq!(args[2], "makepkg");
    }

    #[test]
    fn test_create_makepkg_command_sets_pkg_extensions() {
        let cmd = create_makepkg_command();
        let env_vars: std::collections::HashMap<_, _> = cmd
            .get_envs()
            .filter_map(|(k, v)| v.map(|v| (k, v)))
            .collect();

        // Check that PKGEXT and SRCEXT are set
        assert_eq!(
            env_vars
                .get(std::ffi::OsStr::new("PKGEXT"))
                .map(|s| s.to_string_lossy()),
            Some(std::borrow::Cow::Borrowed(".pkg.tar.zst"))
        );
        assert_eq!(
            env_vars
                .get(std::ffi::OsStr::new("SRCEXT"))
                .map(|s| s.to_string_lossy()),
            Some(std::borrow::Cow::Borrowed(".src.tar.gz"))
        );
    }

    #[test]
    fn test_create_makepkg_command_structure() {
        // Just verify we can create the command without panicking
        let _cmd = create_makepkg_command();
    }
}
