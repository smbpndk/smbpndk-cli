use console::Emoji;

#[cfg(debug_assertions)]
pub const BASE_URL: &'static str = "http://localhost:3000";

#[cfg(not(debug_assertions))]
pub const BASE_URL: &'static str = "https://api.smbpndk.io/";

pub const GUARD_EMOJI: Emoji<'_, '_> = Emoji("🛡  ", "");
