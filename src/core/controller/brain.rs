use super::upload::UploadInterface;
use crate::core::guild::nickname::{GuildNickname, PROPERTIES_STAGING};
use serenity::http::client::Http;
use serenity::model::{guild::PartialGuild, id::GuildId};
use std::fmt;

/// The inner behaviour of a `Controller`. Defines low-level actions such as checking
/// whether the bot is connected to a particular guild.
///
/// In `staging`, the brain is overridden and always acts as though it is connected and
/// points to `Guild(Staging)`. That's the fundamental reason why this is a seperate
/// type - to abstract over the `staging` distinction so high-level code doesn't need
/// to worry about it.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ControllerBrain {
    Global,
    Guild(GuildNickname),
}

impl ControllerBrain {
    /// Returns whether Porygon can connect to the target. In staging, this is
    /// always assumed to be true.
    #[cfg(feature = "staging")]
    pub async fn is_connected(&self, _: &Http) -> bool {
        true
    }

    /// Returns whether Porygon can connect to the target. If the target is `Global`,
    /// this is always true, otherwise it's based on whether the guild can be loaded.
    ///
    /// This is used to determine which setup steps should be run, as setup steps for
    /// un-loaded guilds are not needed.
    #[cfg(not(feature = "staging"))]
    pub async fn is_connected(&self, http: &Http) -> bool {
        match self {
            Self::Global => true,
            Self::Guild(_) => self.try_get_guild(http).await.is_some(),
        }
    }

    /// Returns whether a potential guild ID is a match for this target. All guild IDs
    /// (or lacks thereof) are a match for `staging`.
    #[cfg(feature = "staging")]
    pub fn matches_guild(&self, _: Option<GuildId>) -> bool {
        true
    }

    /// Returns whether a potential guild ID is a match for this target. Always true for
    /// `Global`, otherwise based on whether there is an ID and it matches. This is used
    /// by the event proxy system.
    #[cfg(not(feature = "staging"))]
    pub fn matches_guild(&self, id: Option<GuildId>) -> bool {
        match self {
            Self::Global => true,
            Self::Guild(nick) => match id {
                Some(id) => id == nick.id(),
                None => false,
            },
        }
    }

    /// Fetches the `staging` guild. Despite being an `Option` return to be compatible
    /// with the contract in production, this method can never fail.
    #[cfg(feature = "staging")]
    pub async fn try_get_guild(&self, http: &Http) -> Option<PartialGuild> {
        let guild = PROPERTIES_STAGING
            .get(http)
            .await
            .expect("Staging guild *must* be connected in staging mode!");

        Some(guild)
    }

    /// Fetches the guild for this target. Will be `None` either if called on `Global`, or
    /// if the guild is not connected.
    #[cfg(not(feature = "staging"))]
    pub async fn try_get_guild(&self, http: &Http) -> Option<PartialGuild> {
        match self {
            Self::Global => None,
            Self::Guild(nick) => nick.get(http).await.ok(),
        }
    }

    /// Returns the upload interface for staging, which is always the staging server.
    #[cfg(feature = "staging")]
    pub fn upload_iface(&self) -> UploadInterface {
        UploadInterface::guild(PROPERTIES_STAGING.id())
    }

    /// Returns the upload interface for this target. An upload interface is a low-level
    /// wrapper for uploading, editing, and fetching commands.
    #[cfg(not(feature = "staging"))]
    pub fn upload_iface(&self) -> UploadInterface {
        match self {
            Self::Global => UploadInterface::global(),
            Self::Guild(nick) => UploadInterface::guild(nick.id()),
        }
    }
}

impl fmt::Display for ControllerBrain {
    #[cfg(feature = "staging")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Global => write!(f, "*Global"),
            Self::Guild(nick) => write!(f, "*{nick}"),
        }
    }

    #[cfg(not(feature = "staging"))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Global => write!(f, "Global"),
            Self::Guild(nick) => write!(f, nick),
        }
    }
}
