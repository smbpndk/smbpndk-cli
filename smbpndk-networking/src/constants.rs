#[cfg(debug_assertions)]
pub const BASE_URL: &str = "http://localhost:3000/";

#[cfg(not(debug_assertions))]
pub const BASE_URL: &str = "https://api.smbpndk.com/";
