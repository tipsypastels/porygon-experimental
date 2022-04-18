use serenity::http::client::Http;
use serenity::model::{guild::PartialGuild, id::GuildId};
use std::fmt;

mod properties;

pub use properties::PROPERTIES_STAGING;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GuildNickname {
    Pokecom,
    PokecomStaff,
    DuckCommunism,
}

impl GuildNickname {
    pub fn id(&self) -> GuildId {
        self.properties().id()
    }

    pub fn name(&self) -> &'static str {
        self.properties().name()
    }

    pub async fn get(&self, http: &Http) -> serenity::Result<PartialGuild> {
        self.properties().get(http).await
    }

    fn properties(&self) -> properties::GuildNicknameProperties {
        match self {
            Self::Pokecom => properties::PROPERTIES_POKECOM,
            Self::PokecomStaff => properties::PROPERTIES_POKECOM_STAFF,
            Self::DuckCommunism => properties::PROPERTIES_DUCK_COMMUNISM,
        }
    }
}

impl fmt::Display for GuildNickname {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.properties())
    }
}
