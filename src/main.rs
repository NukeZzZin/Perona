mod commands;
mod utilities;
use std::{
	env::var,
	process::exit,
	sync::Arc,
	time::SystemTime
};
use serenity::{
	async_trait,
	framework::standard::{
		macros::group,
		StandardFramework
	},
	model::prelude::{
		Activity,
		OnlineStatus,
		Ready,
		GatewayIntents,
		GuildId,
		UserId,
		ResumedEvent
	},
	prelude::Context,
	Client as SerenityClient,
	client::EventHandler,
};
use mongodb::{
	Client as MongodbClient,
	options::ClientOptions,
	bson::doc
};
use dotenv::dotenv;
use crate::commands::utilities::*;
use crate::utilities::structures::*;

#[group]
#[description = "👻 Aqui estão algumas funções utilitárias da senhorita Perona 👻"]
#[commands(ping, invite, uptime)]
struct Utilities;

#[derive(Debug)]
struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, context: Context, ready: Ready) {
		let shards = ready.shard.unwrap();
		println!("[+] Perona's was initialized successfully, using shards {}/{} with api version v{}.", shards[0] + 1, shards[1], ready.version);
		context.shard.set_presence(Some(Activity::watching("👻 Hallow-Hallow 👻")), OnlineStatus::DoNotDisturb);
		// TODO: finish implementing ready event.
	}

	async fn cache_ready(&self, _context: Context, _guilds: Vec<GuildId>) {
		println!("[+] Perona's now ready to be used, cache has been fully loaded.");
		// TODO: finish implementing cache_ready event.
	}

	async fn resume(&self, _context: Context, resume: ResumedEvent) {
		resume.trace.into_iter().for_each(|message| {
			println!("[!] resumed after reconnection, logging using trace: {:?}", message);
		});
		// TODO: finish implementing resume event.
	}
}

pub static mut UPTIME: Option<SystemTime> = None;

#[tokio::main]
async fn main() {
	dotenv().expect("[-] Failed to load environment file.");
	let token = var("DISCORD_TOKEN").expect("[-] Failed to find DISCORD_TOKEN in environment file.");
	let application_id = var("APPLICATION_ID").expect("[-] Failed to find APPLICATION_ID in environment file.");
	let database_uri = var("DATABASE_URI").expect("[-] Failed to find DATABASE_URI in environment file.");
	let database_config = ClientOptions::parse(&database_uri).await.unwrap();
	let database_client = MongodbClient::with_options(database_config).unwrap();
	let database_object = database_client.database("database_perona");
	match database_object.run_command(doc!{"ping":1}, None).await {
		Ok(_) => println!("[+] Perona's has been successfully connected to database."),
		Err(why) => {
			eprintln!("[-]  An error occurred while trying to connect to database: {:?}", why);
			exit(0x1);
		}
	}
	let users_collection = database_object.collection::<UsersCollection>("users_perona");
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
	let mut serenity_client = SerenityClient::builder(&token, intents)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("[-] Failed to create serenity client.");
	unsafe {
		UPTIME = Some(SystemTime::now());
		let mut write = serenity_client.data.write().await;
		write.insert::<UsersCollectionContainer>(Arc::new(UsersCollectionContainer::new(users_collection)));
		write.insert::<GuildsCollectionContainer>(Arc::new(GuildsCollectionContainer::new(guilds_collection)));
	}
	if let Err(why) = serenity_client.start_shards(8).await {
		eprintln!("[-] An error occurred while running client: {:?}", why);
		exit(0x1);
	}
}
