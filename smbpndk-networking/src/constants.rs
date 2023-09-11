pub const TOKEN_PATH_STR: &str = ".smb/token";

#[cfg(debug_assertions)]
pub const GH_OAUTH_CLIENT_ID: &str = "053e152f1b78ecee552b";
#[cfg(not(debug_assertions))]
pub const GH_OAUTH_CLIENT_ID: &str = "bf1f12d97659a6495e43";

pub const GH_OAUTH_REDIRECT_HOST: &str = "http://localhost";
pub const GH_OAUTH_REDIRECT_PORT: &str = "8808";

pub const SMB_CLIENT_ID: &str = "cli";
pub const SMB_CLIENT_SECRET: &str = "secretttttttt";

#[cfg(debug_assertions)]
pub const SMB_API_PROTOCOL: &str = "http";
#[cfg(not(debug_assertions))]
pub const SMB_API_PROTOCOL: &str = "https";

#[cfg(debug_assertions)]
pub const SMB_API_HOST: &str = "localhost:8088";
#[cfg(not(debug_assertions))]
pub const SMB_API_HOST: &str = "api.smbpndk.com";
