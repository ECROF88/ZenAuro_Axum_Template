use axum::extract::FromRequestParts;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (用户ID)
    pub sub: String,

    /// Username (用户名)
    pub username: String,

    /// User roles (用户角色)
    pub roles: Vec<UserRole>,

    /// Issued at (签发时间)
    pub iat: u64,

    /// Expiration time (过期时间)
    pub exp: u64,

    /// Not before (生效时间)
    pub nbf: u64,

    /// JWT ID (唯一标识)
    // pub jti: String,

    /// Issuer (签发者)
    pub iss: String,

    /// Audience (受众)
    pub aud: String,
}

impl Claims {
    pub fn new(
        sub: &str,
        username: &str,
        roles: Vec<UserRole>,
        exp_duration: u64,
        // jti: String,
    ) -> Self {
        let now = Utc::now().timestamp() as u64;
        Self {
            sub:sub.to_string(),
            username:username.to_string(),
            roles,
            iat: now,
            exp: now + exp_duration,
            nbf: now,
            // jti,
            iss: "axum-app".to_string(),
            aud: "axum-users".to_string(),
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp() as u64;
        self.exp < now
    }

    pub fn is_valid(&self) -> bool {
        let now = chrono::Utc::now().timestamp() as u64;
        now >= self.nbf && now < self.exp
    }

    pub fn has_role(&self, role: &UserRole) -> bool {
        self.roles.contains(role)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role(&UserRole::Admin)
    }

    pub fn can_perform(&self, required_role: &UserRole) -> bool {
        self.roles
            .iter()
            .any(|role| role.has_permission(required_role))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash,Copy)]
pub enum UserRole {
    Admin,
    User,
    Moderator,
    Guest,
}
impl UserRole {
    pub fn level(&self) -> u8 {
        match self {
            UserRole::Admin => 5,
            UserRole::Moderator => 3,
            UserRole::User => 1,
            UserRole::Guest => 0,
        }
    }
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Moderator => "moderator",
            UserRole::User => "user",
            UserRole::Guest => "guest",
        }
    }

    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "admin" => Some(UserRole::Admin),
            "moderator" => Some(UserRole::Moderator),
            "user" => Some(UserRole::User),
            "guest" => Some(UserRole::Guest),
            _ => None,
        }
    }

    pub fn has_permission(&self, required_role: &UserRole) -> bool {
        self.level() >= required_role.level()
    }
}

// impl FromRequestParts for Claims{
//     type Rejection =AppError;
//     async  fn from_request_parts(parts: &mut axum::http::request::Parts,state: &S,) -> Result<Self,Self::Rejection> {
//         todo!()
//     }
// }
