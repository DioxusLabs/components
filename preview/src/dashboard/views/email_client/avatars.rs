use crate::dashboard::common::{AvatarProfile, AVATAR_PROFILE_OPTIONS};

pub(super) fn avatar_profile_for_key(key: &str) -> &'static AvatarProfile {
    let index = key.bytes().fold(0usize, |hash, byte| {
        hash.wrapping_mul(31).wrapping_add(byte as usize)
    }) % AVATAR_PROFILE_OPTIONS.len();

    &AVATAR_PROFILE_OPTIONS[index]
}
