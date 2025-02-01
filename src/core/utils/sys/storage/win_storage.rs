// TODO: Make win storage actually work

// Win storage is supposed to be a windows only unix_storage.rs
// It is used for minor optimisations, see cofigure.rs

use std::path::Path;

use tracing::warn;

use super::Parallelism;

//TODO: implement this properly
#[must_use]
pub fn parallelism(_: &Path) -> Parallelism {
	warn!("WARNING, parallelism is not going to work on windows");
	Parallelism { nr_requests: None, mq: vec![] }
}
//TODO: implement this
pub fn name_from_path(_: &Path) -> Result<String, &str> {
	Err("TODO: Implement finding of device name on linux")
}
