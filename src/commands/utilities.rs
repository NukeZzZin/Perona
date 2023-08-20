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
#[description("🏓 Este comando calcula a minha latência e retorna o valor.")]
pub async fn ping(context: &Context, message: &Message) -> CommandResult {
	// * it's get gateway latency from elapsed time to message sent.
	let response_latency_start = Instant::now();
	let mut sent_message = message.channel_id.send_message(&context.http, |builder| {
		builder.reference_message(&message.clone());
		builder.content("🏓 Calculando a latência... 🏓");
		return builder;
	}).await.unwrap();
	let response_latency_end = response_latency_start.elapsed();
	// drop(response_latency_start); // * it's drop response_latency_start from memory.
	// * it's get gateway latency from elapsed time in ping geteway.
	let gateway_latency_start = Instant::now();
    context.http.get_gateway().await.unwrap();
    let gateway_latency_end = gateway_latency_start.elapsed();
	// drop(gateway_latency_start); // * it's drop gateway_latency_start from memory.
	let embed_content = perona_default_embed(&context,
		"👻 Informações sobre a minha latência 👻",
		format!("🎈 Latência do getaway: **_`{}ms`_**.\n🔥 Latência da api: **_`{}ms`_**.",
			gateway_latency_end.as_millis(),
			response_latency_end.as_millis())
	).await;
	sent_message.edit(&context.http, |edit| {
		edit
			.content('\u{0000}') // * it's set content with null byte.
			.embed(|embed| {
				embed.clone_from(&embed_content);
				return embed;
			});
		return edit;
	}).await.unwrap();
	return CommandResult::Ok(());
}

#[command]
#[description("💌 Este comando gera um dos meus convite.")]
pub async fn invite(context: &Context, message: &Message) -> CommandResult {
	let embed_content = perona_default_embed(&context,
		"👻 Link para convidar me para seu servidor 👻",
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
#[description("📅 Este comando calcula meu uptime e retorna o valor.")]
pub async fn uptime(context: &Context, message: &Message) -> CommandResult {
	let embed_content;
	unsafe {
		let time = UPTIME.unwrap().elapsed().unwrap().as_millis(); // * it's get process uptime as milliseconds from memory.
		embed_content = perona_default_embed(&context,
			"👻 Informações sobre o meu tempo de atividade 👻",
			format!("🕗 O meu tempo de atividade: **_`{:02}d:{:02}h:{:02}m:{:02}s`_**.",
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
	// TODO: finish implementing help command (permissions, etc).
	if arguments.is_empty() == true {
		let mut embed_content = perona_default_embed(&context,
			"👻 Aqui estão todos meus comandos 👻",
			"📜 Caso queria informações específicas de algum comando use: **_`P!help <command>`_**."
		).await;
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
			embed_content.field(format!("_{}_", group.options.description.unwrap_or(group.name)), buffer, false);
		}
		message.channel_id.send_message(&context.http, |builder| {
			builder.reference_message(&message.clone());
			builder.embed(|embed| {
				embed.clone_from(&embed_content);
				return embed;
			});
			return builder;
		}).await.unwrap();
		return CommandResult::Ok(());
	} else {
		let argument = arguments.single::<String>().unwrap();
		let mut command = None;
		for group in groups.iter() {
			for commands in group.options.commands.iter() {
				if commands.options.names.iter().any(|liter| liter == &argument.to_lowercase()) {
					command = Some(commands);
				}
			}
		}
		match command {
			None => {
				let embed_content = perona_default_embed(&context,
					"👻 Me desculpe este comando não existe 👻",
					"📜 Tente usar **_`P!help`_** para ver todos meus comandos"
				).await;
				message.channel_id.send_message(&context.http, |builder| {
					builder.reference_message(&message.clone());
					builder.embed(|embed| {
						embed.clone_from(&embed_content);
						return embed;
					});
					return builder;
				}).await.unwrap();
				return CommandResult::Ok(());
			},
			Some(command) => {
				let mut embed_content = perona_default_embed(&context,
					format!("👻 Hellow-Hellow 👻"),
					format!("📜 Aqui estão algumas informações mais específicas sobre o comando: **_`{}`_**.", command.options.names[0])
				).await;
				if let Some(description) = command.options.desc {
					embed_content.field("👀 Descrição:", format!("**_`{}`_**", description), true);
				}
				if command.options.names.len() > 1 {
					let buffer = command.options.names
						.iter()
						.skip(1)
						.fold(String::new(), |_result, item| {
							return format!("- **_`{}`_**\n", item);
						});
					embed_content.field("💡 Aliases:", buffer, true);
				}
				// embed_content.field('\u{200b}', '\u{200b}', false); // * it's break field column.
				if let Some(usage) = command.options.usage {
					embed_content.field("🔧 Uso:", format!("**_`P!{}`_**", usage), true);
				}
				// if command.options.examples.is_empty() == false {
				// 	let buffer = command.options.examples
				// 		.iter()
				// 		.fold(String::new(), |_result, item| {
				// 			return format!("- **_`P!{}`_**\n", item);
				// 		});
				// 	embed_content.field("💻 Exemplos", buffer, true);
				// }
				message.channel_id.send_message(&context.http, |builder| {
					builder.reference_message(&message.clone());
					builder.embed(|embed| {
						embed.clone_from(&embed_content);
						return embed;
					});
					return builder;
				}).await.unwrap();
				return CommandResult::Ok(());
			}
		}
	}
}
