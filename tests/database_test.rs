use mongodb::{
	Client as MongodbClient,
	options::ClientOptions,
	bson::{
		doc,
		Bson,
		Document
	}
};
use dotenv::dotenv;
use std::env::var;

#[cfg(test)]
mod database_test {
	use super::*;
	#[tokio::test]
	async fn connect_test() {
		dotenv().ok();
		let database_uri = var("DATABASE_TEST_URI").expect("[-] Failed to find DATABASE_URI in environment file.");
		let database_config = ClientOptions::parse(&database_uri).await.unwrap();
		let database_client = MongodbClient::with_options(database_config).unwrap();
		let database_object = database_client.database("database_test");
		let connection_callback = database_object.run_command(doc!{"ping":1}, None).await.unwrap();
		assert_eq!(connection_callback.get("ok").and_then(Bson::as_i32), Some(1));
	}

	#[tokio::test]
	async fn interactions_test() {
		dotenv().ok();
		let database_uri = var("DATABASE_TEST_URI").expect("[-] Failed to find DATABASE_URI in environment file.");
		let database_config = ClientOptions::parse(&database_uri).await.unwrap();
		let database_client = MongodbClient::with_options(database_config).unwrap();
		let database_object = database_client.database("database_test");
		let database_collection = database_object.collection::<Document>("collection_test");
		let docuemnt_test = doc!{"key":"value"};

		// * it simulate insert operation
		database_collection.insert_one(docuemnt_test.clone(), None).await.unwrap();
		let insert_one_callback = database_collection.find_one(docuemnt_test.clone(), None).await.unwrap().unwrap();
		assert_eq!(insert_one_callback.is_empty(), false);

		// * it simulate find operation
		let find_one_callback = database_collection.find_one(docuemnt_test.clone(), None).await.unwrap().unwrap();
		assert_eq!(find_one_callback.get_str("key").unwrap(), "value");

		// * it simulate update operation
		database_collection.update_one(docuemnt_test.clone(), doc!{"$set":{"new_key":"new_value"}}, None).await.unwrap();
		let update_one_callback = database_collection.find_one(docuemnt_test.clone(), None).await.unwrap().unwrap();
		assert_eq!(update_one_callback.get_str("new_key").unwrap(), "new_value");

		// * it simulate delete operation
		database_collection.delete_one(docuemnt_test.clone(), None).await.unwrap();
		let delete_one_callback = database_collection.find_one(docuemnt_test.clone(), None).await.unwrap();
		assert_eq!(delete_one_callback.is_none(), true)
	}
}
