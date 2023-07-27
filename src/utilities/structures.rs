use mongodb::{
	bson::{
		to_bson,
		Bson,
		Document
	},
	options::{
		DeleteOptions,
		InsertOneOptions,
		UpdateOptions,
		FindOneAndUpdateOptions,
		FindOneOptions
	},

	Collection
};
use serde::{
	Deserialize,
	Serialize
};
use std::{
	sync::Arc,
	collections::HashMap
};
use serenity::{
	prelude::TypeMapKey,
	model::id::UserId,
	model::id::GuildId
};
use tokio::sync::RwLock;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsersCollection {
	#[serde(rename="_id")]
	pub user_id: GuildId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildsCollection {
	#[serde(rename="_id")]
	pub guild_id: UserId,
}

#[derive(Debug)]
pub struct UsersCollectionContainer {
	pub data: Collection<UsersCollection>
}

#[derive(Debug)]
pub struct GuildsCollectionContainer {
	pub data: Collection<GuildsCollection>
}

#[derive(Debug, Clone)]
pub struct GuildsCollectionRuntime {
	pub data: Arc<RwLock<HashMap<String, GuildsCollection>>>
}

#[allow(dead_code)]
impl GuildsCollectionRuntime {
	pub fn new() -> Self {
        return GuildsCollectionRuntime {
			data: Arc::new(RwLock::new(HashMap::new()))
		};
    }

	pub async fn get_data(&self, key: String) -> Option<GuildsCollection> {
		return self.data.read().await.get(&key).cloned();
    }

	pub async fn insert_or_update_data(&self, key: String, value: GuildsCollection) -> Option<GuildsCollection> {
		return self.data.write().await.insert(key, value);
    }

    pub async fn remove_data(&self, key: String) -> Option<GuildsCollection> {
		return self.data.write().await.remove(&key);
    }
}

#[allow(dead_code)]
impl UsersCollectionContainer {
	pub fn new(collection: Collection<UsersCollection>) -> Self {
		return UsersCollectionContainer {
			data: collection
		};
	}

	pub async fn find_one(&self, filter: Document, options: Option<FindOneOptions>) -> Option<UsersCollection> {
		return self.data.find_one(filter, options).await.unwrap();
	}

	pub async fn update_data(&self, query: Document, value: GuildsCollection, options: Option<UpdateOptions>) {
		if let Bson::Document(document) = to_bson(&value).unwrap() {
			self.data.update_one(query, document, options).await.unwrap();
		}
	}

	pub async fn update_many_data(&self, query: Document, update: Document, options: Option<UpdateOptions>) {
		self.data.update_many(query, update, options).await.unwrap();
	}

	pub async fn find_one_and_update_data(&self, filter: Document, update: Document, options: Option<FindOneAndUpdateOptions>) -> Option<UsersCollection> {
		return self.data.find_one_and_update(filter, update, options).await.unwrap();
	}

	pub async fn insert_data(&self, value: UsersCollection, options:  Option<InsertOneOptions>) {
		self.data.insert_one(value, options).await.unwrap();
	}

	pub async fn delete_data(&self, query: Document, options: Option<DeleteOptions>) {
		self.data.delete_one(query, options).await.unwrap();
	}
}

#[allow(dead_code)]
impl GuildsCollectionContainer {
	pub fn new(collection: Collection<GuildsCollection>) -> Self {
		return GuildsCollectionContainer {
			data: collection
		};
	}

	pub async fn find_one(&self, filter: Document, options: Option<FindOneOptions>) -> Option<GuildsCollection> {
		return self.data.find_one(filter, options).await.unwrap();
	}

	pub async fn update_data(&self, query: Document, value: GuildsCollection, options: Option<UpdateOptions>) {
		if let Bson::Document(document) = to_bson(&value).unwrap() {
			self.data.update_one(query, document, options).await.unwrap();
		}
	}

	pub async fn update_many_data(&self, query: Document, update: Document, options: Option<UpdateOptions>) {
		self.data.update_many(query, update, options).await.unwrap();
	}

	pub async fn find_one_and_update_data(&self, filter: Document, update: Document, options: Option<FindOneAndUpdateOptions>) -> Option<GuildsCollection> {
		return self.data.find_one_and_update(filter, update, options).await.unwrap();
	}

	pub async fn insert_data(&self, value: GuildsCollection, options:  Option<InsertOneOptions>) {
		self.data.insert_one(value, options).await.unwrap();
	}

	pub async fn delete_data(&self, query: Document, options: Option<DeleteOptions>) {
		self.data.delete_one(query, options).await.unwrap();
	}
}

impl TypeMapKey for UsersCollectionContainer {
	type Value = Arc<UsersCollectionContainer>;
}

impl TypeMapKey for GuildsCollectionRuntime {
	type Value = Arc<GuildsCollectionRuntime>;
}

impl TypeMapKey for GuildsCollectionContainer {
	type Value = Arc<GuildsCollectionContainer>;
}
