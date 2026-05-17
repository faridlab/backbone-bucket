//! Local extension trait that adds `has_permission` / `has_role` / `has_all_roles`
//! to `backbone_auth::AuthContext`.
//!
//! Bridges generator-emitted call sites (`auth.has_permission("X")`,
//! `auth.has_role("Y")`) over a `backbone-auth` version whose `AuthContext`
//! exposes only the raw `permissions: Vec<String>` and `roles: Vec<String>`
//! fields.

use backbone_auth::middleware::AuthContext;

pub trait AuthContextExt {
    fn has_permission(&self, permission: &str) -> bool;
    fn has_role(&self, role: &str) -> bool;
    fn has_all_roles(&self, roles: &[&str]) -> bool;
}

impl AuthContextExt for AuthContext {
    fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|p| p == permission)
    }

    fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    fn has_all_roles(&self, roles: &[&str]) -> bool {
        roles.iter().all(|required| self.roles.iter().any(|r| r == required))
    }
}
