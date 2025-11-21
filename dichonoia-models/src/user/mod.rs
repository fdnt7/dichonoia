pub mod discriminator;

use serde::{Deserialize, Serialize};

use crate::{id::UserId, user::discriminator::Discriminator};

// TODO: Finish rest of the struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    #[serde(with = "crate::user::discriminator")]
    pub discriminator: Option<Discriminator>,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
}
