mod commands;
mod utilities;
use std::{
	env::var,
	sync::Arc,
	time::SystemTime
};
use serenity::{
	async_trait,
	framework::standard::{
		macros::{
			group,
			hook
		},
		StandardFramework,
		CommandResult,
		DispatchError
	},
	model::prelude::{
		channel::Message,
		Activity,
		OnlineStatus,
		Ready,
		GatewayIntents,
		GuildId,
		UserId,
		ResumedEvent
	},
	Client as SerenityClient,
	client::EventHandler,
	prelude::Context,
};
use mongodb::{
	Client as MongodbClient,
	options::ClientOptions,
	bson::doc
};
use crate::utilities::structures::{
	UsersCollection,
	UsersCollectionContainer,
	GuildsCollection,
	GuildsCollectionContainer
};
use crate::utilities::functions::{
	PeronaLoggerStatus,
	perona_default_embed
};
use dotenv::dotenv;
use crate::commands::funny::*;
use crate::commands::moderation::*;
use crate::commands::utilities::*;

#[group]
#[description = "👯 Diversão - Este módulo possui alguns comandos que vão divertir todos do servidor:"]
#[commands(dice)]
struct Funny;

#[group]
#[description = "⚖️ Moderação - Este módulo possui alguns comandos que vão te ajudar a moderar seu servidor:"]
#[commands(ban, kick)]
struct Moderation;

#[group]
#[description = "👷 Utilitários - Este módulo possui alguns comandos diversos:"]
#[commands(ping, invite, uptime)]
struct Utilities;

#[derive(Debug)]
struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, context: Context, ready: Ready) {
		let shards = ready.shard.unwrap();
		perona_println!(PeronaLoggerStatus::Info, "Perona's was initialized successfully, using shards {}/{} with api version v{}", shards[0] + 1, shards[1], ready.version);
		context.shard.set_presence(Some(Activity::watching("👻 Hallow-Hallow 👻")), OnlineStatus::DoNotDisturb);
		// TODO: finish implementing ready event.
	}

	async fn cache_ready(&self, _context: Context, _guilds: Vec<GuildId>) {
		perona_println!(PeronaLoggerStatus::Info, "Perona's now ready to be used, cache has been fully loaded");
		// TODO: finish implementing cache_ready event.
	}

	async fn resume(&self, _context: Context, resume: ResumedEvent) {
		resume.trace.into_iter().for_each(|message| {
			perona_println!(PeronaLoggerStatus::Warning, "Perona's shard resumed after reconnection, logging using trace: {:#?}", message);
		});
		// TODO: finish implementing resume event.
	}
}

#[hook]
async fn dispatch_error(context: &Context, message: &Message, error: DispatchError, command: &str) {
	match error {
		DispatchError::Ratelimited(timeout) => {
			let embed_content = perona_default_embed(&context,
				"👻 Não foi possível executar esté comando 👻",
				format!("💔 Perece que você excedeu o meu **_`rate limit`_**.\n🩹 Por favor aguarde **_`{} segundos`_**.", timeout.as_secs())
			).await;
			message.channel_id.send_message(&context.http, |builder| {
				builder.content(&message.author);
				builder.reference_message(&message.clone());
				builder.embed(|embed| {
					embed.clone_from(&embed_content);
					return embed;
				});
				return builder;
			}).await.unwrap();
		},
		DispatchError::LackingPermissions(permissions) => {
			let embed_content = perona_default_embed(&context,
				"👻 Não foi possível executar esté comando 👻",
				format!("💔 Perece que você não tem as permissões necessárias.\n🩹 Para executar este comando você precisar ter essas permissões: **_`{}`_**", permissions)
			).await;
			message.channel_id.send_message(&context.http, |builder| {
				builder.content(&message.author);
				builder.reference_message(&message.clone());
				builder.embed(|embed| {
					embed.clone_from(&embed_content);
					return embed;
				});
				return builder;
			}).await.unwrap();
		}
		DispatchError::TooManyArguments { max, given } => {
			let embed_content = perona_default_embed(&context,
				"👻 Não foi possível executar esté comando 👻",
				format!("💔 Tente remover argumentos desnecessários.\n🩹 Foram recebidos **_`{}`_** argumentos e tem como máximo de **_`{}`_**.", given, max)
			).await;
			message.channel_id.send_message(&context.http, |builder| {
				builder.content(&message.author);
				builder.reference_message(&message.clone());
				builder.embed(|embed| {
					embed.clone_from(&embed_content);
					return embed;
				});
				return builder;
			}).await.unwrap();
		}
		DispatchError::NotEnoughArguments { min, given } => {
			let embed_content = perona_default_embed(&context,
				"👻 Não foi possível executar esté comando 👻",
				format!("💔 Tente adicionar os argumentos necessários.\n🩹 Foram recebidos **_`{}`_** argumentos e são necessários **_`{}`_**.", given, min)
			).await;
			message.channel_id.send_message(&context.http, |builder| {
				builder.content(&message.author);
				builder.reference_message(&message.clone());
				builder.embed(|embed| {
					embed.clone_from(&embed_content);
					return embed;
				});
				return builder;
			}).await.unwrap();
		},
		DispatchError::OnlyForDM => {
			let embed_content = perona_default_embed(&context,
				"👻 Não foi possível executar esté comando 👻",
				"💔 Esté comando é uso exclusivo dos para canal **_`DM`_**.\n"
			).await;
			message.channel_id.send_message(&context.http, |builder| {
				builder.content(&message.author);
				builder.reference_message(&message.clone());
				builder.embed(|embed| {
					embed.clone_from(&embed_content);
					return embed;
				});
				return builder;
			}).await.unwrap();
		}
		DispatchError::OnlyForGuilds => {
			let embed_content = perona_default_embed(&context,
				"👻 Não foi possível executar esté comando 👻",
				"💔 Esté comando é uso exclusivo dos para canal **_`Guild`_**.\n"
			).await;
			message.channel_id.send_message(&context.http, |builder| {
				builder.content(&message.author);
				builder.reference_message(&message.clone());
				builder.embed(|embed| {
					embed.clone_from(&embed_content);
					return embed;
				});
				return builder;
			}).await.unwrap();
		}
		DispatchError::OnlyForOwners => {
			let embed_content = perona_default_embed(&context,
				"👻 Não foi possível executar esté comando 👻",
				"💔 Esté comando é uso exclusivo dos meus **_`desenvolvedores`_**.\n"
			).await;
			message.channel_id.send_message(&context.http, |builder| {
				builder.content(&message.author);
				builder.reference_message(&message.clone());
				builder.embed(|embed| {
					embed.clone_from(&embed_content);
					return embed;
				});
				return builder;
			}).await.unwrap();
		},
		_ => {
			perona_println!(PeronaLoggerStatus::Error, "An error occurred while running command: [{}]: {:#?}", command, error)
		}
	}
	// TODO: finish implementing dispatch_error hook
}

#[hook]
async fn after_hook(_context: &Context, _message: &Message, command: &str, result: CommandResult) {
	if let Err(why) = result {
		perona_println!(PeronaLoggerStatus::Error, "An error occurred while running command: [{}]: {:#?}", command, why);
	}
	// TODO: finish implementing after hook
}

pub static mut UPTIME: Option<SystemTime> = None;

#[tokio::main]
async fn main() {
	// * it's defines some things necessary for full work of application.
	unsafe {
		UPTIME = Some(SystemTime::now());
		// let trace = std::backtrace::Backtrace::force_capture();
		// TODO: implement hook for logger.
	}
	dotenv().ok();
	let token = var("DISCORD_TOKEN").expect("[-] Failed to find DISCORD_TOKEN in environment file");
	let application_id = var("APPLICATION_ID").expect("[-] Failed to find APPLICATION_ID in environment file");
	let application_owners = var("APPLICATION_OWNERS").expect("[-] Failed to find APPLICATION_OWNERS in environment file");
	let database_uri = var("DATABASE_URI").expect("[-] Failed to find DATABASE_URI in environment file");
	let database_config = ClientOptions::parse(&database_uri).await.unwrap();
	let database_client = MongodbClient::with_options(database_config).unwrap();
	let database_object = database_client.database("database_perona");
	match database_object.run_command(doc!{"ping":1}, None).await {
		Ok(_) => perona_println!(PeronaLoggerStatus::Info, "Perona's has been successfully connected to database"),
		Err(why) => {
			perona_println!(PeronaLoggerStatus::Fatal, "An error occurred while trying to connect to database: {:#?}", why);
		}
	}
	let users_collection = database_object.collection::<UsersCollection>("users_perona");
	let guilds_collection = database_object.collection::<GuildsCollection>("guilds_perona");
	let framework = StandardFramework::new()
		.configure(|configuraion| {
			configuraion
				.prefix("P!")
				.allow_dm(true)
				.ignore_webhooks(true)
				.ignore_bots(true)
				.with_whitespace(false)
				.case_insensitivity(true)
				.owners(vec![UserId(application_owners.parse::<u64>().unwrap())].into_iter().collect())
				.on_mention(Some(UserId(application_id.parse::<u64>().unwrap())));
			return configuraion;
		})
		.after(after_hook)
		.on_dispatch_error(dispatch_error)
		.group(&FUNNY_GROUP)
		.group(&MODERATION_GROUP)
		.group(&UTILITIES_GROUP)
		.help(&HELP);
	let intents = GatewayIntents::all();
	let mut serenity_client = SerenityClient::builder(&token, intents)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("Failed to create serenity client");
	{
		let mut write = serenity_client.data.write().await;
		write.insert::<UsersCollectionContainer>(Arc::new(UsersCollectionContainer::new(users_collection)));
		write.insert::<GuildsCollectionContainer>(Arc::new(GuildsCollectionContainer::new(guilds_collection)));
	}
	if let Err(why) = serenity_client.start_autosharded().await {
		perona_println!(PeronaLoggerStatus::Fatal, "An error occurred while running client: {:#?}", why);
	}
}
