use clap::Parser;
use conduwuit::Result;

use crate::{
	appservice::{self, AppserviceCommand}, check::{self, CheckCommand}, command::Command, debug::{self, DebugCommand}, dns::{self, DNSCommand}, federation::{self, FederationCommand}, media::{self, MediaCommand}, query::{self, QueryCommand}, room::{self, RoomCommand}, server::{self, ServerCommand}, user::{self, UserCommand}
};

#[derive(Debug, Parser)]
#[command(name = "conduwuit", version = conduwuit::version())]
pub(super) enum AdminCommand {
	#[command(subcommand)]
	/// - Commands for managing appservices
	Appservices(AppserviceCommand),

	#[command(subcommand)]
	/// - Commands for managing local users
	Users(UserCommand),

	#[command(subcommand)]
	/// - Commands for managing rooms
	Rooms(RoomCommand),

	#[command(subcommand)]
	/// - Commands for managing federation
	Federation(FederationCommand),

	#[command(subcommand)]
	/// - Commands for managing the server
	Server(ServerCommand),

	#[command(subcommand)]
	/// - Commands for managing media
	Media(MediaCommand),

	#[command(subcommand)]
	/// - Commands for checking integrity
	Check(CheckCommand),

	#[command(subcommand)]
	/// - Commands for debugging things
	Debug(DebugCommand),

	#[command(subcommand)]
	/// - Low-level queries for database getters and iterators
	Query(QueryCommand),
	#[command(subcommand)]
	/// - Commands for messing with conduwuit's resolution system
	DNS(DNSCommand)
}

#[tracing::instrument(skip_all, name = "command")]
pub(super) async fn process(command: AdminCommand, context: &Command<'_>) -> Result {
	use AdminCommand::*;

	match command {
		| Appservices(command) => appservice::process(command, context).await?,
		| Media(command) => media::process(command, context).await?,
		| Users(command) => user::process(command, context).await?,
		| Rooms(command) => room::process(command, context).await?,
		| Federation(command) => federation::process(command, context).await?,
		| Server(command) => server::process(command, context).await?,
		| Debug(command) => debug::process(command, context).await?,
		| Query(command) => query::process(command, context).await?,
		| Check(command) => check::process(command, context).await?,
		| DNS(command) => dns::process(command, context).await?,
	};

	Ok(())
}
