use serenity::{
	framework::standard::{
		macros::{
			command,
			help
		},
		CommandResult,
		HelpOptions,
		CommandGroup,
		Args
	},
	model::{
		prelude::{
			channel::Message,
			UserId
		},
		Permissions
	},
	prelude::Context
};
use crate::{
	utilities::functions::perona_default_embed,
	UPTIME
};
use std::collections::HashSet;
use tokio::time::Instant;

#[command]
#[aliases("latency")]
pub async fn ping(context: &Context, message: &Message) -> CommandResult {
	// * it's get gateway latency from elapsed time to message sent.
	let response_latency_start = Instant::now();
	let mut response = message.channel_id.send_message(&context.http, |builder| {
		builder.reference_message(&message.clone());
		builder.content("🏓 Calculando a latência... 🏓");
		return builder;
	}).await.unwrap();
	let response_latency_end = response_latency_start.elapsed();
	drop(response_latency_start); // * it's drop response_latency_start from memory.
	// * it's get gateway latency from elapsed time in ping geteway.
	let gateway_latency_start = Instant::now();
    context.http.get_gateway().await.unwrap();
    let gateway_latency_end = gateway_latency_start.elapsed();
	drop(gateway_latency_start); // * it's drop gateway_latency_start from memory.
	let embed_content = perona_default_embed(&context,
		"👻 Informações sobre a latência da Perona 👻",
		format!("🎈 Latência do getaway : **_`{}ms`_**.\n🔥 Latência da api: **_`{}ms`_**.",
			gateway_latency_end.as_millis(),
			response_latency_end.as_millis())
	).await;
	response.edit(&context.http, |edit| {
		edit
			.content('\u{0}') // * it's set content with null byte.
			.embed(|embed| {
				embed.clone_from(&embed_content);
				return embed;
			});
		return edit;
	}).await.unwrap();
	return CommandResult::Ok(());
}

#[command]
pub async fn invite(context: &Context, message: &Message) -> CommandResult {
	let embed_content = perona_default_embed(&context,
		"👻 Link para convidar a Perona para seu servidor 👻",
		format!("❤️ Me convide para seu servidor utilizando este link: ***{}***.",
			context.http.get_current_user().await.unwrap().invite_url(&context.http, Permissions::all()).await.unwrap()) // * it's generate invite link with all permissions.
	).await;
	message.channel_id.send_message(&context.http, |builder| {
		builder.embed(|embed| {
			embed.clone_from(&embed_content);
			return embed;
		});
		return builder;
	}).await.unwrap();
	return CommandResult::Ok(());
}

#[command]
pub async fn uptime(context: &Context, message: &Message) -> CommandResult {
	let embed_content;
	unsafe {
		let time = UPTIME.unwrap().elapsed().unwrap().as_millis(); // * it's get process uptime as milliseconds from memory.
		embed_content = perona_default_embed(&context,
			"👻 Informações sobre o tempo de atividade da Perona 👻",
			format!("🕗 O tempo de atividade da Perona: **_`{:02}d:{:02}h:{:02}m:{:02}s`_**.",
				time / 86400, // * it's format day.
				(time % 86400) / 3600, // * it's format hours.
				(time % 3600) / 60, // * it's format minutes.
				time % 60) // * it's format seconds.
		).await;
	}
	message.channel_id.send_message(&context.http, |builder| {
		builder.embed(|embed| {
			embed.clone_from(&embed_content);
			return embed;
		});
		return builder;
	}).await.unwrap();
	return CommandResult::Ok(());
}

#[help]
pub async fn help(context: &Context, message: &Message, mut arguments: Args, _options: &'static HelpOptions, groups: &[&'static CommandGroup], _users: HashSet<UserId>) -> CommandResult {
	// TODO: finish implementing help command.
	if arguments.is_empty() {
		let mut embed_content = perona_default_embed(&context, "👻 Aqui estão todos comandos da Perona 👻", "📜 Caso queria informações específicas de algum comando use: **_`P!help <command>`_**.").await;
		for group in groups.iter() {
			if group.options.help_available == false {
				continue;
			}
			let mut buffer = String::new();
			for commands in group.options.commands.iter() {
				if commands.options.help_available {
					buffer.push_str(format!(" **_`{}`_**", commands.options.names.first().unwrap()).as_str()); // * it's push all commands into buffer.
				}
			}
			embed_content.field(group.options.description.unwrap_or(group.name), buffer, false);
		}
		message.channel_id.send_message(&context.http, |builder| {
			builder.reference_message(&message.clone());
			builder.embed(|embed| {
				embed.clone_from(&embed_content);
				return embed;
			});
			return builder;
		}).await.unwrap();
	}
	return CommandResult::Ok(());
}
