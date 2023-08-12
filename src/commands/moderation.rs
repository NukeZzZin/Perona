use serenity::{
	framework::standard::{
		macros::command,
		CommandResult,
		Args
	},
	model::prelude::{
		channel::Message,
		UserId
	},
	prelude::Context
};
use crate::{
	utilities::functions::{
		perona_default_embed,
		PeronaLoggerStatus
	},
	perona_println
};

// TODO: implement various commands for moderation

#[command]
#[only_in(guilds)]
#[required_permissions(BAN_MEMBERS)]
#[min_args(1)]
pub async fn ban(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
	let duration = arguments.single::<u8>().unwrap_or(0);
	let user_id = arguments.single::<UserId>().unwrap();
	let reason = arguments.remains();
	if let Ok(member) = message.guild_id.unwrap().member(&context, user_id).await {
		if let Some(reason) = reason {
			if let Err(why) = member.ban_with_reason(&context, duration, reason).await {
				perona_println!(PeronaLoggerStatus::Error, "An error occurred while running command: {:#?}", why);
				let embed_content = perona_default_embed(&context,
					String::from("👻 Não foi possível banir este membro 👻"),
					String::from("❌ Ao tentar banir este membro enfrentei alguns problemas.")
				).await;
				message.channel_id.send_message(&context.http, |message| {
					message.embed(|embed| {
						embed.clone_from(&embed_content);
						return embed;
					});
					return message;
				}).await.unwrap();
			} else {
					let embed_content = perona_default_embed(&context,
						String::from("👻 O membro foi banido pela Perona 👻"),
						format!("🔨 Membro foi banido pelo ID : **_`{}`_**.\n📅 Membro foi banido pelo tempo : **_`{} dias`_**.\n📜 Membro foi banido pelo movito : **_`{}`_**.",
							user_id.0, duration, reason)
					).await;
					message.channel_id.send_message(&context.http, |message| {
						message.embed(|embed| {
							embed.clone_from(&embed_content);
							return embed;
						});
						return message;
					}).await.unwrap();
				}
			} else {
				if let Err(why) = member.ban(&context, duration).await {
					perona_println!(PeronaLoggerStatus::Error, "An error occurred while running command: {:#?}", why);
					let embed_content = perona_default_embed(&context,
						String::from("👻 Não foi possível banir este membro 👻"),
						String::from("❌ Ao tentar banir este membro enfrentei alguns problemas.")
					).await;
					message.channel_id.send_message(&context.http, |message| {
						message.embed(|embed| {
							embed.clone_from(&embed_content);
							return embed;
						});
						return message;
					}).await.unwrap();
				} else {
					let embed_content = perona_default_embed(&context,
						String::from("👻 O membro foi banido pela Perona 👻"),
						format!("🔨 Membro foi banido pelo ID : **_`{}`_**.\n📅 Membro foi banido pelo tempo : **_`{} dias`_**.",
							user_id.0, duration)
					).await;
					message.channel_id.send_message(&context.http, |message| {
						message.embed(|embed| {
							embed.clone_from(&embed_content);
							return embed;
						});
						return message;
					}).await.unwrap();
				}
			}
		} else {
			let embed_content = perona_default_embed(&context,
				String::from("👻 Não foi possível banir este membro 👻"),
				String::from("❌ Não foi possível banir este membro, pois esté membro não existe no servidor.")
			).await;
			message.channel_id.send_message(&context.http, |message| {
				message.embed(|embed| {
					embed.clone_from(&embed_content);
					return embed;
				});
				return message;
			}).await.unwrap();
		}
	return CommandResult::Ok(());
}

#[command]
#[only_in(guilds)]
#[required_permissions(KICK_MEMBERS)]
#[min_args(1)]
pub async fn kick(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
	let user_id = arguments.single::<UserId>().unwrap();
	let reason = arguments.remains();
	if let Ok(member) = message.guild_id.unwrap().member(&context, user_id).await {
		if let Some(reason) = reason {
			if let Err(why) = member.kick_with_reason(&context.http, reason).await {
				perona_println!(PeronaLoggerStatus::Error, "An error occurred while running command: {:#?}", why);
				let embed_content = perona_default_embed(&context,
					String::from("👻 Não foi possível expulsar este membro 👻"),
					String::from("🩹 Ao tentar expulsar este membro enfrentei alguns problemas.")
				).await;
				message.channel_id.send_message(&context.http, |message| {
					message.embed(|embed| {
						embed.clone_from(&embed_content);
						return embed;
					});
					return message;
				}).await.unwrap();
			} else {
				let embed_content = perona_default_embed(&context,
					String::from("👻 O membro foi banido pela Perona 👻"),
					format!("🔨 Membro foi expulso pelo ID : **_`{}`_**.\n📜 Membro foi expulso pelo movito : **_`{}`_**.",
						user_id.0, reason)
				).await;
				message.channel_id.send_message(&context.http, |message| {
					message.embed(|embed| {
						embed.clone_from(&embed_content);
						return embed;
					});
					return message;
				}).await.unwrap();
			}
		} else {
			if let Err(why) = member.kick(&context.http).await {
				perona_println!(PeronaLoggerStatus::Error, "An error occurred while running command: {:#?}", why);
				let embed_content = perona_default_embed(&context,
					String::from("👻 Não foi possível expulsar este membro 👻"),
					String::from("🩹 Ao tentar expulsar este membro enfrentei alguns problemas.")
				).await;
				message.channel_id.send_message(&context.http, |message| {
					message.embed(|embed| {
						embed.clone_from(&embed_content);
						return embed;
					});
					return message;
				}).await.unwrap();
			} else {
				let embed_content = perona_default_embed(&context,
					String::from("👻 O membro foi expulso pela Perona 👻"),
					format!("🔨 Membro foi expulso pelo ID : **_`{}`_**.",
						user_id.0)
				).await;
				message.channel_id.send_message(&context.http, |message| {
					message.embed(|embed| {
						embed.clone_from(&embed_content);
						return embed;
					});
					return message;
				}).await.unwrap();
			}
		}
	}
	return CommandResult::Ok(());
}
