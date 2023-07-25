use serenity::prelude::TypeMapKey;
use mongodb::{
	bson::{
		to_bson,
		doc,
		Bson,
		Document
	},
	options::{
		DeleteOptions,
		InsertOneOptions,
		UpdateOptions,
		FindOneAndUpdateOptions,
		FindOneOptions,
		ReturnDocument
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
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildsCollection {
	pub guild_id: String,
	pub last_message: String
}

#[derive(Debug)]
pub struct GuildsCollectionContainer {
	data: Collection<GuildsCollection>
}

#[derive(Debug)]
pub struct GuildsCollectionRuntime {
	data: Arc<RwLock<HashMap<String, GuildsCollection>>>
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

	pub async fn insert_or_update_data(&self, key: String, value: GuildsCollection) {
		self.data.write().await.insert(key, value);
    }

    pub async fn remove_data(&self, key: String) {
		self.data.write().await.remove(&key);
    }
}

#[allow(dead_code)]
impl GuildsCollectionContainer {
	pub fn new(collection: Collection<GuildsCollection>) -> Self {
		return GuildsCollectionContainer {
			data: collection
		};
	}

	pub async fn get_data(&self, filter: Document, options: Option<FindOneOptions>) -> Option<GuildsCollection> {
		return self.data.find_one(filter, options).await.unwrap();
	}

	pub async fn update_data(&self, query: Document, value: GuildsCollection, options: Option<UpdateOptions>) {
		if let Bson::Document(document) = to_bson(&value).unwrap() {
			self.data.update_one(query, document, options).await.unwrap();
		}
	}

	pub async fn insert_or_update_data(&self, filter: Document, update: Document) {
		self.data.find_one_and_update(filter, update, FindOneAndUpdateOptions::builder()
			.upsert(true)
			.return_document(ReturnDocument::After)
			.build()
		).await.unwrap();
	}

	pub async fn insert_data(&self, value: GuildsCollection, options:  Option<InsertOneOptions>) {
		self.data.insert_one(value, options).await.unwrap();
	}

	pub async fn delete_data(&self, query: Document, options: Option<DeleteOptions>) {
		self.data.delete_one(query, options).await.unwrap();
	}
}

impl TypeMapKey for GuildsCollectionRuntime {
	type Value = Arc<GuildsCollectionRuntime>;
}

impl TypeMapKey for GuildsCollectionContainer {
	type Value = Arc<GuildsCollectionContainer>;
}
