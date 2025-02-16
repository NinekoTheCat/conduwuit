mod commands;

use clap::Subcommand;
use conduwuit::Result;
use ruma::ServerName;

use crate::admin_command_dispatch;

#[admin_command_dispatch]
#[derive(Debug, Subcommand)]
pub(super) enum DNSCommand {
    /// - List cached entries in the DNS resolver
    CachedEntries {
        server_name: Box<ServerName>
    }
}