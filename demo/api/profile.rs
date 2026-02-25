use metaxy::rpc_query;
use serde::Serialize;

/// User role with snake_case serialization.
/// Produces: `"admin"`, `"power_user"`, `"guest"`.
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Admin,
    PowerUser,
    /// Serialized as `"anonymous"` instead of `"guest"`.
    #[serde(rename = "anonymous")]
    Guest,
}

/// Account event kind with kebab-case serialization.
/// Produces: `"sign-in"`, `"sign-out"`, `"password-reset"`.
#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventKind {
    SignIn,
    SignOut,
    PasswordReset,
}

/// A user profile with camelCase fields in JSON.
///
/// Demonstrates `rename_all`, per-field `rename`, `skip`, and `default`.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub user_id: u64,
    pub display_name: String,
    pub email_address: String,
    pub role: UserRole,
    pub last_event: EventKind,

    /// Overrides camelCase — appears as `"profile_url"` in JSON.
    #[serde(rename = "profile_url")]
    pub profile_url: String,

    /// Never included in the JSON response.
    #[serde(skip)]
    pub internal_score: f64,

    /// Optional — omitted from JSON when `None` and not provided.
    #[serde(default)]
    pub avatar_url: Option<String>,
}

/// Look up a user profile by ID.
///
/// Showcases serde attributes: `rename_all`, `rename`, `skip`, `default`
/// on structs and enums to demonstrate TypeScript codegen fidelity.
#[rpc_query]
async fn profile(user_id: u64) -> UserProfile {
    UserProfile {
        user_id,
        display_name: "Alice".to_string(),
        email_address: "alice@example.com".to_string(),
        role: UserRole::Admin,
        last_event: EventKind::SignIn,
        profile_url: "https://example.com/alice".to_string(),
        internal_score: 42.0,
        avatar_url: Some("https://example.com/alice/avatar.png".to_string()),
    }
}
