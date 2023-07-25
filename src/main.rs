mod commands;
mod utilities;
use std::{
	env,
	error::Error,
	sync::Arc
};
use serenity::{
	async_trait,
	framework::standard::{
		macros::group,
		StandardFramework
	},
	client::EventHandler,
	prelude::*,
	model::prelude::*
};
use mongodb::options::ClientOptions;
use crate::commands::utilities::*;
use crate::utilities::structures::*;
#[group]
#[description = "👻 Aqui estão algumas funções utilitárias da senhorita Perona 👻"]
#[commands(ping, invite, source)]
struct Utilities;
#[derive(Debug)]
struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, context: Context, ready: Ready) {
		println!("[+] Perona's was initialized successfully, using shards {}/{} with api version v{}.", ready.shard.unwrap()[0] + 1, ready.shard.unwrap()[1], ready.version);
		context.shard.set_presence(Some(Activity::playing("👻 que tal tentar digitar P!help 👻")), OnlineStatus::DoNotDisturb);
		// * finish implementing ready event.
	}

	async fn message(&self, context: Context, message: Message) {
		let data = context.data.read().await;
		if let Some(guild_collection) = data.get::<GuildsCollectionRuntime>() {
			guild_collection.insert_or_update_data(message.guild_id.unwrap().0.to_string(), GuildsCollection {
				guild_id: message.guild_id.unwrap().0.to_string(),
				last_message: message.content,
			}).await;
			println!("{:?}", guild_collection.get_data(message.guild_id.unwrap().0.to_string()).await);
		}
	}

	async fn cache_ready(&self, _context: Context, _guilds: Vec<GuildId>) {
		println!("[+] Perona's now ready to be used, the cache has been fully loaded.");
		// * finish implementing cache_ready event.
	}

	async fn resume(&self, _context: Context, resume: ResumedEvent) {
		resume.trace.into_iter().for_each(|message| {
			dbg!("[!] resumed after reconnection, logging using trace: {}", Some(message));
		});
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	dotenv::dotenv().expect("[-] Failed to load environment file.");
	let token = env::var("DISCORD_TOKEN").expect("[-] Failed to find DISCORD_TOKEN in environment file.");
	let application_id = env::var("APPLICATION_ID").expect("[-] Failed to find APPLICATION_ID in environment file.");
	let database_uri = env::var("DATABASE_URI").expect("[-] Failed to find DATABASE_URI in environment file.");
	let database_config = ClientOptions::parse(&database_uri).await.unwrap();
	let database_client = mongodb::Client::with_options(database_config).unwrap();
	let database_object = database_client.database("database_perona");
	let guilds_collection = database_object.collection::<GuildsCollection>("guilds_perona");
	let framework = StandardFramework::new()
		.configure(|configuraion| {
			configuraion
				.with_whitespace(false)
				.prefix("P!")
				.case_insensitivity(true)
				.on_mention(Some(UserId(application_id.parse::<u64>().unwrap())))
		})
		.group(&UTILITIES_GROUP);
	let intents = GatewayIntents::all();
	let mut serenity_client = serenity::Client::builder(&token, intents)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("[-] Failed to create serenity client.");
	{
		let mut data = serenity_client.data.write().await;
		data.insert::<GuildsCollectionRuntime>(Arc::new(GuildsCollectionRuntime::new()));
		data.insert::<GuildsCollectionContainer>(Arc::new(GuildsCollectionContainer::new(guilds_collection)));
	}
	if let Err(why) = serenity_client.start_shards(8).await {
		eprintln!("[-] An error occurred while running client: {:?}", why);
	}
	return Ok(())
}
