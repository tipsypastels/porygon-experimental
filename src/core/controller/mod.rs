use crate::core::guild::nickname::GuildNickname;
use brain::ControllerBrain;
use serenity::http::client::Http;
use serenity::model::{guild::PartialGuild, id::GuildId};
use std::fmt;

mod brain;
mod upload;

/// A part of Porygon's setup process. Controllers are unique objects that
/// specify targets to upload and setup against. For example, each guild has
/// a controller that can be used as the target of a command upload, as can
/// `GLOBAL` for uploads without a specific guild.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Controller {
    brain: ControllerBrain,
}

impl Controller {
    /// Returns whether Porygon can connect to the controller's guild, if the
    /// controller describes a guild, or `true` if the controller is `GLOBAL`.
    pub async fn is_connected(&self, http: &Http) -> bool {
        self.brain.is_connected(http).await
    }

    /// Tests whether a hypothetical guild ID (or lack of one) matches a controller.
    /// This is used by the event proxy system to decide whether to delegate events
    /// to an specific guild's event handler.
    pub fn matches_guild(&self, guild_id: Option<GuildId>) -> bool {
        self.brain.matches_guild(guild_id)
    }

    /// Tries to look up the guild for this controller. Will be `None` either if called
    /// on `GLOBAL` or if the guild is not connected.
    pub async fn try_get_guild(&self, http: &Http) -> Option<PartialGuild> {
        self.brain.try_get_guild(http).await
    }

    /// Returns the low-level upload interface to allow editing and fetching commands
    /// during the setup process.
    pub fn upload_iface(&self) -> upload::UploadInterface {
        self.brain.upload_iface()
    }
}

impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.brain)
    }
}

impl fmt::Debug for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Controller").field(&self.brain).finish()
    }
}

/// The `GLOBAL` controller for uploading to the global target.
pub const GLOBAL: Controller = Controller {
    brain: ControllerBrain::Global,
};

/// The `POKECOM` controller for uploading to PokéCommunity public discord.
pub const POKECOM: Controller = Controller {
    brain: ControllerBrain::Guild(GuildNickname::Pokecom),
};

/// The `POKECOM_STAFF` controller for uploading to PokéCommunity staff discord.
pub const POKECOM_STAFF: Controller = Controller {
    brain: ControllerBrain::Guild(GuildNickname::PokecomStaff),
};

/// The `DUCK_COMMUNISM` controller for uploading to Duck Communism private discord.
pub const DUCK_COMMUNISM: Controller = Controller {
    brain: ControllerBrain::Guild(GuildNickname::DuckCommunism),
};
