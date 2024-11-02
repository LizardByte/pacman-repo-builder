use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Default, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BuildMetadata {
    SrcInfo,
    PkgBuild,
    #[default]
    Either,
}
