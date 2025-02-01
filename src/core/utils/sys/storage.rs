// windows and unix systems handle storage radically differently

#[cfg(target_os="windows")]
#[path ="./storage/win_storage.rs"]
 mod store;

#[cfg(not(target_os="windows"))]
#[path ="./storage/unix_storage.rs"]
 mod store;

 /// Device characteristics useful for random access throughput
#[derive(Clone, Debug, Default)]
pub struct Parallelism {
	/// Number of requests for the device.
	pub nr_requests: Option<usize>,

	/// Individual queue characteristics.
	pub mq: Vec<Queue>,
}

/// Device queue characteristics
#[derive(Clone, Debug, Default)]
pub struct Queue {
	/// Queue's indice.
	pub id: usize,

	/// Number of requests for the queue.
	pub nr_tags: Option<usize>,

	/// CPU affinities for the queue.
	pub cpu_list: Vec<usize>,
}
 pub use self::store::*;