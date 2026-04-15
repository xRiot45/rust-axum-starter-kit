//! Integration and unit tests.
//!
//! Run with: `cargo test`
//! For integration tests with a real DB, set TEST_DATABASE_URL env var.

#[cfg(test)]
mod user_tests {
    /// Example: test that the UserProfile conversion strips the password hash.
    #[test]
    fn user_profile_hides_password() {
        use crate::modules::users::domain::model::{User, UserProfile};
        use chrono::Utc;
        use uuid::Uuid;

        let user = User {
            id: Uuid::new_v4(),
            name: "Alice".into(),
            email: "alice@example.com".into(),
            password_hash: "secret_hash".into(),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let profile: UserProfile = user.into();
        let json = serde_json::to_string(&profile).unwrap();
        assert!(!json.contains("secret_hash"), "password_hash must not appear in JSON");
    }
}
