use super::super::{
    args::PatchMakepkgArgs,
    status::{Code, Failure, Status},
    utils::{MakepkgPatch, MAKEPKG_PATCHES},
};
use hex_fmt::HexFmt;
use pipe_trait::Pipe;
use std::fs::{read, write};

pub fn patch_makepkg(args: PatchMakepkgArgs) -> Status {
    let PatchMakepkgArgs {
        replace,
        unsafe_ignore_unknown_changes,
    } = args;

    let makepkg = match read("/usr/bin/makepkg") {
        Ok(content) => content,
        Err(error) => {
            eprintln!("⮾ {}", error);
            return error.pipe(Failure::from).into();
        }
    };

    let patch = match (
        MakepkgPatch::find_patch(&MAKEPKG_PATCHES, &makepkg),
        unsafe_ignore_unknown_changes,
    ) {
        (Ok(patch), _) => patch,
        (Err(actual_hash), false) => {
            eprintln!("🛈 sha1sum of expected default system makepkg:");
            for patch in &MAKEPKG_PATCHES {
                eprintln!("  → {}", HexFmt(patch.original_sha1sum));
            }
            eprintln!("🛈 sha1sum of custom makepkg:");
            for patch in &MAKEPKG_PATCHES {
                eprintln!("  → {}", HexFmt(patch.custom_sha1sum));
            }
            eprintln!("🛈 sha1sum of actual system makepkg:");
            eprintln!("  → {}", HexFmt(actual_hash));
            eprintln!("⮾ makepkg had been modified by an unknown party");
            eprintln!("⮾ it is not safe to proceed");
            eprintln!("🛈 run again with --unsafe-ignore-unknown-changes to ignore this error");
            return Code::UnrecognizedMakepkg.into();
        }
        (Err(_), true) => *MAKEPKG_PATCHES.last().unwrap(),
    };

    if !unsafe_ignore_unknown_changes {
        if let Err(actual_hash) = MakepkgPatch::find_patch(&MAKEPKG_PATCHES, &makepkg) {
            eprintln!("🛈 sha1sum of expected default system makepkg:");
            for patch in &MAKEPKG_PATCHES {
                eprintln!("  → {}", HexFmt(patch.original_sha1sum));
            }
            eprintln!("🛈 sha1sum of custom makepkg:");
            for patch in &MAKEPKG_PATCHES {
                eprintln!("  → {}", HexFmt(patch.custom_sha1sum));
            }
            eprintln!("🛈 sha1sum of actual system makepkg:");
            eprintln!("  → {}", HexFmt(actual_hash));
            eprintln!("⮾ makepkg had been modified by an unknown party");
            eprintln!("⮾ it is not safe to proceed");
            eprintln!("🛈 run again with --unsafe-ignore-unknown-changes to ignore this error");
            return Code::UnrecognizedMakepkg.into();
        }
    }

    if replace {
        if let Err(error) = write("/usr/bin/makepkg", patch.custom_content) {
            eprintln!("⮾ {}", error);
            return error.pipe(Failure::from).into();
        }
    } else {
        print!("{}", patch.custom_content);
        eprintln!();
        eprintln!("# NOTE: Above is the content of custom makepkg script");
        eprintln!("# NOTE: Run again with --replace flag to replace system's makepkg");
    }

    Ok(())
}
