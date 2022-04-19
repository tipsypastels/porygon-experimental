use serenity::http::client::Http;
use serenity::model::{
    guild::{Guild, PartialGuild},
    id::GuildId,
};
use std::fmt;

/// The properties of Porygon's member guilds, which are always hardcoded into the bot
/// and can be used as targets for features directly.
///
/// `GuildNicknameProperties` is *internal* and should be accessed through `GuildNickname`.
/// The exception is `PROPERTIES_STAGING`, which is not selectable that way and exported
/// directly instead, since it can never be used as a chosen guild directly.
#[derive(Debug)]
pub struct GuildNicknameProperties {
    id: GuildId,
    name: &'static str,
}

/// Properties for the PokéCommunity public server.
pub const PROPERTIES_POKECOM: GuildNicknameProperties = GuildNicknameProperties {
    id: GuildId(157983957902819328),
    name: "Pokecom",
};

/// Properties for the PokéCommunity Staff private server.
pub const PROPERTIES_POKECOM_STAFF: GuildNicknameProperties = GuildNicknameProperties {
    id: GuildId(193103073210662914),
    name: "PokcomStaff",
};

/// Properties for the Duck Communism private server.
pub const PROPERTIES_DUCK_COMMUNISM: GuildNicknameProperties = GuildNicknameProperties {
    id: GuildId(322199235825238017),
    name: "DuckCommunism",
};

/// Properties for the override staging server.
/// Only exists when compiled in staging mode.
///
/// Unlike the other `GuildNicknameProperties`, this one is exported publicly
/// from its parent module, since it needs to be accessible directly. It is not
/// accessible via the wrapping enum `GuildNickname`, because it is never a valid
/// option in the places that enum is used.
#[cfg(feature = "staging")]
pub const PROPERTIES_STAGING: GuildNicknameProperties = GuildNicknameProperties {
    id: GuildId(964389981516881920),
    name: "Staging",
};

impl GuildNicknameProperties {
    pub fn id(&self) -> GuildId {
        self.id
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub async fn get(&self, http: &Http) -> serenity::Result<PartialGuild> {
        Guild::get(http, self.id).await
    }
}

impl fmt::Display for GuildNicknameProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
