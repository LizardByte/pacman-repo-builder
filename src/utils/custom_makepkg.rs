use hex_literal::hex;
use sha2::{digest::Output, Digest, Sha256};

#[derive(Debug, Clone, Copy)]
pub struct MakepkgPatch {
    pub custom_content: &'static str,
    pub custom_sha256sum: [u8; 32],
    pub original_sha256sum: [u8; 32],
}

impl MakepkgPatch {
    pub fn find_patch(list: &[Self], original_content: &[u8]) -> Result<Self, Output<Sha256>> {
        let hash = Sha256::digest(original_content);
        list.iter()
            .copied()
            .find(|patch| {
                patch.original_sha256sum == hash.as_slice()
                    || patch.custom_sha256sum == hash.as_slice()
            })
            .ok_or(hash)
    }
}

pub const MAKEPKG_PATCHES: [MakepkgPatch; 7] = [
    MakepkgPatch {
        custom_content: include_str!("./patches/makepkg5"),
        custom_sha256sum: hex!("cf9afaccb49ef852cc8d0ba5977f9cd0dabee57e0e3ac5108846dcb7a1cd7937"),
        // pacman-5.2.2-1
        original_sha256sum: hex!(
            "46bcdfd2dcee085e0af808a23819959b919b646874b7a21fdf6b8c2d4070048c"
        ),
    },
    MakepkgPatch {
        custom_content: include_str!("./patches/makepkg6"),
        custom_sha256sum: hex!("57e7a7976fc8b9f9f909475234a96ef23dc4a946da572b30d24e2e566dcc19d0"),
        // pacman-6.0.0-1
        original_sha256sum: hex!(
            "aaed7e7adbed3e7340fcfc92098181ed78bca510565003390e123b41a52c3a86"
        ),
    },
    MakepkgPatch {
        custom_content: include_str!("./patches/makepkg6"),
        custom_sha256sum: hex!("57e7a7976fc8b9f9f909475234a96ef23dc4a946da572b30d24e2e566dcc19d0"),
        // pacman-6.0.1-1
        original_sha256sum: hex!(
            "4951379ce18dec36ef077557e2765ee76fefcd6bce75d7897bc6bdece7e943b5"
        ),
    },
    MakepkgPatch {
        custom_content: include_str!("./patches/makepkg6"),
        custom_sha256sum: hex!("57e7a7976fc8b9f9f909475234a96ef23dc4a946da572b30d24e2e566dcc19d0"),
        // pacman-6.0.2-1
        original_sha256sum: hex!(
            "f06df9f6efe840d10efbe37b489e0d008fa4a28d562cef5a72480bbce4eb0eb5"
        ),
    },
    MakepkgPatch {
        custom_content: include_str!("./patches/makepkg6"),
        custom_sha256sum: hex!("57e7a7976fc8b9f9f909475234a96ef23dc4a946da572b30d24e2e566dcc19d0"),
        // pacman-6.1.0-1
        original_sha256sum: hex!(
            "e4453e78e964be351820d7370e6138490bc222f8fcbb960b45d2cdb8dbfdefed"
        ),
    },
    MakepkgPatch {
        custom_content: include_str!("./patches/makepkg7"),
        custom_sha256sum: hex!("7c5c46a2b92efbb67f69ec4c54130d1aa5be87d44b2e33e6f9664404c9e75f96"),
        // pacman-7.0.0.r1.g7cf2b01-1
        original_sha256sum: hex!(
            "5d50987152a311e455e92c3e75612c431fd6be2c1b4b2879e7947e5297df9c6f"
        ),
    },
    MakepkgPatch {
        custom_content: include_str!("./patches/makepkg8"),
        custom_sha256sum: hex!("acf14363bb73e2fb14c49047fa44ad3bdf064d49a5177f81965a771dfd20e948"),
        // pacman-7.1.0.r7.gb9f7d4a-1
        original_sha256sum: hex!(
            "ecc42cabafaa51b0f84084c2f9eeee3c358d62fe533cb4002207f95c84e486b1"
        ),
    },
];

#[cfg(test)]
mod tests {
    use super::{MakepkgPatch, MAKEPKG_PATCHES};

    #[test]
    fn recognizes_bundled_makepkg_scripts() {
        for patch in MAKEPKG_PATCHES {
            let matched =
                MakepkgPatch::find_patch(&MAKEPKG_PATCHES, patch.custom_content.as_bytes())
                    .unwrap();
            assert_eq!(matched.custom_content, patch.custom_content);
        }
    }

    #[test]
    fn rejects_unknown_makepkg_scripts() {
        assert!(MakepkgPatch::find_patch(&MAKEPKG_PATCHES, b"unknown makepkg script").is_err());
    }
}
