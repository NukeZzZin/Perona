use serenity::{
	framework::standard::{
		macros::command,
		CommandResult
	},
	model::{
		prelude::channel::Message,
		Permissions
	},
	prelude::Context
};
use crate::{
	utilities::functions::perona_default_embed,
	UPTIME
};
use tokio::time::Instant;

#[command]
#[aliases("latency")]
pub async fn ping(context: &Context, message: &Message) -> CommandResult {
	// * it's get gateway latency from elapsed time to message sent.
	let response_latency_start = Instant::now();
	let mut response = message.channel_id.send_message(&context.http, |message| {
		message.content("🏓 Calculando a latência... 🏓");
		return message;
	}).await.unwrap();
	let response_latency_end = response_latency_start.elapsed();
	drop(response_latency_start); // * it's drop response_latency_start from memory.
	// * it's get gateway latency from elapsed time in ping geteway.
	let gateway_latency_start = Instant::now();
    context.http.get_gateway().await.unwrap();
    let gateway_latency_end = gateway_latency_start.elapsed();
	drop(gateway_latency_start); // * it's drop gateway_latency_start from memory.
	let embed_content = perona_default_embed(&context,
		String::from("👻 Informações sobre a latência da Perona 👻"),
		format!("🎈 Latência do getaway : **_`{}ms`_**.\n🔥 Latência da api : **_`{}ms`_**.",
			gateway_latency_end.as_millis(),
			response_latency_end.as_millis())
	).await;
	response.edit(&context.http, |edit| {
		edit
			.content(b'\0') // * it's set content with null byte.
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
		String::from("👻 Link para convidar a Perona para seu servidor 👻"),
		format!("❤️ Me convide para seu servidor utilizando este link : ***{}***.",
			context.http.get_current_user().await.unwrap().invite_url(&context.http, Permissions::all()).await.unwrap())
	).await;
	message.channel_id.send_message(&context.http, |message| {
		message.embed(|embed| {
			embed.clone_from(&embed_content);
			return embed;
		});
		return message;
	}).await.unwrap();
	return CommandResult::Ok(());
}

#[command]
pub async fn uptime(context: &Context, message: &Message) -> CommandResult {
	let embed_content;
	unsafe {
		let time = UPTIME.unwrap().elapsed().unwrap().as_millis();
		embed_content = perona_default_embed(&context,
			String::from("👻 Informações sobre o tempo de atividade da Perona 👻"),
			format!("🕗 O tempo de atividade da Perona : **_`{:02}d:{:02}h:{:02}m:{:02}s`_**.",
				time / 86400,
				(time % 86400) / 3600,
				(time % 3600) / 60,
				time % 60)
		).await;
	}
	message.channel_id.send_message(&context.http, |message| {
		message.embed(|embed| {
			embed.clone_from(&embed_content);
			return embed;
		});
		return message;
	}).await.unwrap();
	return CommandResult::Ok(());
}
