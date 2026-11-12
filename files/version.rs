//! Static build info for Native Instruments Traktor Pro 4.

pub const BUILD_VERSION: &str  = "v3.7.28";
pub const BUILD_CHANNEL: &str  = "lts";
pub const BUILD_COMMIT: &str   = "5a89ee233404";
pub const BUILD_TIMESTAMP: u64 = 1755655719;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Native Instruments Traktor Pro 4", BUILD_VERSION, BUILD_CHANNEL)
}
