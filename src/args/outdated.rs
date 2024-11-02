use argh::*;
use std::str::FromStr;

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "outdated", description = "List outdated packages")]
pub struct OutdatedArgs {
    #[argh(
        option,
        description = "level of details of information (pkgname|pkg-file-path|lossy-yaml|strict-yaml)"
    )]
    pub details: Option<OutdatedDetails>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub enum OutdatedDetails {
    PkgName,
    #[default]
    PkgFilePath,
    LossyYaml,
    StrictYaml,
}

impl FromStr for OutdatedDetails {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(match text {
            "pkgname" => OutdatedDetails::PkgName,
            "pkg-file-path" => OutdatedDetails::PkgFilePath,
            "lossy-yaml" => OutdatedDetails::LossyYaml,
            "strict-yaml" => OutdatedDetails::StrictYaml,
            _ => return Err(format!("invalid choice: {}", text)),
        })
    }
}
