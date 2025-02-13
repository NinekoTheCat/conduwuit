use std::{marker::PhantomData, sync::Arc};

use serde::{de::DeserializeOwned, Serialize};

use crate::{Engine, Map, Result as ConduwuitResult};
/// A [Arc<Map>] that accepts a key and value
/// you can obtain it by calling [`CborMap::open`]
/// K is the database Key
/// V is the database Value
/// if the key can be turned into a reference to [u8] then you should use the Raw functions
pub struct CborMap<K, V> {
	map: Arc<Map>,
	types: PhantomData<(K, V)>,
}
impl<K, V> CborMap<K, V> {
	/// Remember to always put the name of a new table you've created into
	/// [`crate::maps::MAPS`]
	pub fn open(db: &Arc<Engine>, name: &'static str) -> ConduwuitResult<Arc<Self>> {
		let map = Map::open(db, name)?;
		Ok(Arc::new(CborMap { map, types: PhantomData }))
	}
    /// Gets the raw [crate::Map]
	pub fn get_underlying_map(&self) -> Arc<Map> { self.map.to_owned() }
}

/// complex keys :3
impl<K, V> CborMap<K, V>
where
	K: Serialize,
	V: Serialize,
{
}
/// getting a value
impl<K, V> CborMap<K, V>
where
	K: Serialize,
	V: DeserializeOwned,
{
	/// Gets a value from the database or None if it doesn't exist
	pub async fn get_value_from_key(&self, key: &K) -> Option<V> {
		let serialised_key = minicbor_serde::to_vec(key).expect("key to be serialisable");
		let undecoded_value = self.map.get(&serialised_key).await.ok()?;
		let decoded_value: V = minicbor_serde::from_slice::<V>(&undecoded_value).ok()?;
		Some(decoded_value)
	}
}
use std::fmt::Debug;
impl<K, V> CborMap<K, V>
where
	K: AsRef<[u8]> + Debug + Sized,
	V: DeserializeOwned + Clone,
{
	/// Gets the value from a key, None otherwise.
	pub async fn get_value_from_raw_key(&self, key: &K) -> Option<V> {
		let undecoded_value = self.map.get(key).await.ok()?;
		let decoded_value: V = minicbor_serde::from_slice::<V>(&undecoded_value).ok()?;
		Some(decoded_value)
	}
}

impl<K, V> CborMap<K, V>
where
	K: Serialize,
{
	/// Removes a key-value pair from the database, where K is the key in the
	/// database
	pub fn remove_key(&mut self, key: K) {
		let serialised_key = minicbor_serde::to_vec(key).expect("key to be serialisable");
		self.map.remove(&serialised_key);
	}
}

impl<K, V> CborMap<K, V>
where
	K: AsRef<[u8]> + Debug + Sized,
{
	/// Removes a key-value pair from the database, where K is the key in the
	/// database
	pub fn remove_raw_key(&mut self, key: K) { self.map.remove(&key); }
}

impl<K, V> CborMap<K, V>
where
	K: AsRef<[u8]> + Debug + Sized,
	V: Serialize,
{
	/// Inserts a value where the key of the database points, overwrites
	/// existing values if they already existed where the key was if you want
	/// to know whenever or not it exists at the key use
	/// [`CborMap::get_value_from_raw_key`]
	pub fn insert_with_raw_key(&mut self, key: &K, value: V) {
		let serialised_value = minicbor_serde::to_vec(value).expect("value to be serialisable");
		self.map.insert(key, serialised_value);
	}
}

/// complex keys :3
impl<K, V> CborMap<K, V>
where
	K: Serialize,
	V: Serialize,
{
	/// Inserts a value where the key of the database points, overwrites
	/// existing values if they already existed where the key was if you want
	/// to know whenever or not it exists at the key use
	/// [`CborMap::get_value_from_key`]
	pub fn insert_at_key(&mut self, key: &K, value: V) {
		let (serialised_key, serialised_value) =
			Self::expect_serialisable(key, value).expect("we should be able to serialize this");
		self.map.insert(&serialised_key, serialised_value);
	}

	fn expect_serialisable(
		key: &K,
		value: V,
	) -> Result<(Vec<u8>, Vec<u8>), minicbor_serde::error::EncodeError<std::convert::Infallible>>
	{
		let serialised_key = minicbor_serde::to_vec(key)?;
		let serialised_value = minicbor_serde::to_vec(value)?;
		Ok((serialised_key, serialised_value))
	}
}
