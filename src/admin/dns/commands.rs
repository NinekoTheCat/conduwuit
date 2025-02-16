
use conduwuit::{
	utils::time::{self},
	warn, Result,
};
use futures::StreamExt;
use futures::TryFutureExt;
use ruma::{
	events::room::message::RoomMessageEventContent,
	ServerName,
};

use crate::admin_command;
#[admin_command]
pub(super) async fn cached_entries(&self, server_name: Box<ServerName>) -> Result<RoomMessageEventContent> {
	let resolver = self.services.resolver.clone();
	let destination = resolver
		.cache
		.get_destination(&server_name)
		.map_err(|x| {
			warn!("Could not find destination because: {}", x);
			x
		})
		.await
		.ok();
	let dest_override = resolver
		.cache
		.get_override(server_name.as_str())
		.map_err(|x| {
			warn!("Could not find override because: {}", x);
			x
		})
		.await
		.ok();
	if let Some(destination) = destination {
        let expires_at: String = time::format(destination.expire, "%+");
		self.write_str(&format!(
			r#"
### Cached Destination
|Server Name|Host|Expires At|
|-|-|-|
|{}|{}|{}|
"#,
			destination.dest,
			destination.host,
			expires_at
        )).await?;
	}
	if let Some(dest_override) = dest_override {
        let expires_at: String = time::format(dest_override.expire, "%+");

        self.write_str(&format!(
			r#"
### Cached Destination IP Override
|Server Name|Port|Expires at| Ips|
|-|-|-|-|
|{}|{}|{}|[{}]|
 "#,
			dest_override.overriding.unwrap_or("N/A".to_string()),
			dest_override.port,
			expires_at,
			dest_override
				.ips
				.iter()
				.map(|x| format!("{}", x.to_string()))
				.collect::<Vec<_>>()
				.join(",")
		)).await?;
	}
	Ok(RoomMessageEventContent::notice_plain(""))
}
