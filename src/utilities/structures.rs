use serde::{
	Deserialize,
	Serialize
};
use mongodb::{
	Collection,
	bson::DateTime
};
use std::sync::Arc;
use serenity::prelude::TypeMapKey;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsersCollection {
	#[serde(rename="_id")]
	pub user_id: String,
	pub created_at: DateTime
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildsCollection {
	#[serde(rename="_id")]
	pub guild_id: String,
	pub guild_prefix: String,
	pub created_at: DateTime
}

#[derive(Debug)]
pub struct UsersCollectionContainer {
	pub	data: Collection<UsersCollection>
}

#[derive(Debug)]
pub struct GuildsCollectionContainer {
	pub data: Collection<GuildsCollection>
}

#[allow(dead_code)]
impl UsersCollection {
	pub fn new(user_id: String) -> Self {
		return UsersCollection {
			user_id: user_id,
			created_at: DateTime::now()
		}
	}
}

#[allow(dead_code)]
impl GuildsCollection {
	pub fn new(guild_id: String) -> Self {
		return GuildsCollection {
			guild_id: guild_id,
			guild_prefix: String::from("P!"),
			created_at: DateTime::now()
		}
	}
}

impl UsersCollectionContainer {
	pub fn new(collection: Collection<UsersCollection>) -> Self {
		return UsersCollectionContainer {
			data: collection
		};
	}
}

impl GuildsCollectionContainer {
	pub fn new(collection: Collection<GuildsCollection>) -> Self {
		return GuildsCollectionContainer {
			data: collection
		};
	}
}

impl TypeMapKey for UsersCollectionContainer {
	type Value = Arc<UsersCollectionContainer>;
}

impl TypeMapKey for GuildsCollectionContainer {
	type Value = Arc<GuildsCollectionContainer>;
}
