use console::Emoji;

#[cfg(debug_assertions)]
pub const BASE_URL: &str = "http://localhost:3000";

#[cfg(not(debug_assertions))]
pub const BASE_URL: &str = "https://api.smbpndk.com";

pub const OK_EMOJI: Emoji<'_, '_> = Emoji("✅ ", "");
pub const ERROR_EMOJI: Emoji<'_, '_> = Emoji("❌ ", "");
