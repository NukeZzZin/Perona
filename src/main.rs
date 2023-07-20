mod commands;
mod utilities;
use std::{
	env,
	error::Error
};
use serenity::{
	async_trait,
	framework::standard::{
		macros::group,
		StandardFramework
	},
	client::EventHandler,
	model::id::UserId,
	prelude::*,
	model::prelude::*
};
use crate::commands::utilities::*;

#[group]
#[description = "👻 Aqui estão algumas funções utilitárias da senhorita Perona 👻"]
#[commands(ping, invite, source)]
struct Utilities;

#[derive(Debug)]
struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, context: Context, _ready: Ready) {
		println!("[+] Perona's was initialized successfully, using shared id {}.", context.shard_id);
		context.shard.set_presence(Some(Activity::playing("👻 que tal tentar digitar P!help 👻")), OnlineStatus::DoNotDisturb);
		// * finish implementing ready event.
	}

	async fn resume(&self, _context: Context, resume: ResumedEvent) {
		println!("[!] resumed after reconnection, using trace {:?}", resume.trace);
		// * finish implementing resume event.
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	dotenv::dotenv().expect("[-] Failed to load environment file.");
	let token = env::var("DISCORD_TOKEN").expect("[-] Failed to find DISCORD_TOKEN in environment file.");
	let application_id = env::var("APPLICATION_ID").expect("[-] Failed to find APPLICATION_ID in environment file.");
	let framework = StandardFramework::new()
		.configure(|conf| {
			conf
				.with_whitespace(false)
				.prefix("P!")
				.on_mention(Some(UserId(application_id.parse::<u64>().unwrap())))
				.case_insensitivity(true)
		})
		.group(&UTILITIES_GROUP);
	let intents = GatewayIntents::all();
	let mut client = Client::builder(&token, intents)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("[-] Failed to create serenity client.");
	if let Err(why) = client.start_shards(8).await {
		eprintln!("[-] An error occurred while running client: {:?}", why);
	}
	Ok(())
}
