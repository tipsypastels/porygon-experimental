use serde_json::value::Value;
use serenity::http::client::Http;
use serenity::model::{
    id::{CommandId, GuildId},
    interactions::application_command::ApplicationCommand,
};
use serenity::Result;

/// A low-level interface for uploading commands. Controller brains
/// provide `upload_iface` that will get you one of these.
///
/// This type is extremely low-level and wraps Serenity's own low-level
/// facilities for command uploading. There are higher-level versions of
/// this in Serenity, but none of them do exactly what I want.
pub struct UploadInterface(UploadInterfaceInner);

/// Inner type to prevent constructing an upload iface outside the parent scope.
#[derive(Debug)]
enum UploadInterfaceInner {
    Global,
    Guild(GuildId),
}

use UploadInterfaceInner::*;

impl UploadInterface {
    /// Creates an upload interface for a given guild.
    pub(super) fn guild(id: GuildId) -> Self {
        Self(Guild(id))
    }

    /// Creates an upload interface for the global target.
    pub(super) fn global() -> Self {
        Self(Global)
    }

    /// Uploads the given command data. If `command_id` is provided, it will be edited.
    /// Otherwise, it will be uploaded as a new command.
    pub async fn upload(
        &self,
        http: &Http,
        data: &Value,
        command_id: Option<CommandId>,
    ) -> Result<ApplicationCommand> {
        match (&self.0, command_id) {
            // Editing a global command.
            (Global, Some(cid)) => http.edit_global_application_command(cid.0, data).await,

            // Creating a new global command.
            (Global, None) => http.create_global_application_command(data).await,

            // Editing a guild command.
            (Guild(gid), Some(cid)) => {
                http.edit_guild_application_command(gid.0, cid.0, data)
                    .await
            }

            // Creating a new guild command.
            (Guild(gid), None) => http.create_guild_application_command(gid.0, data).await,
        }
    }

    /// Gets a command with a specific ID.
    pub async fn get(&self, http: &Http, command_id: CommandId) -> Result<ApplicationCommand> {
        match &self.0 {
            Global => http.get_global_application_command(command_id.0).await,
            Guild(guild_id) => {
                http.get_guild_application_command(guild_id.0, command_id.0)
                    .await
            }
        }
    }
}
