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
use tokio::time::Instant;
use crate::utilities::functions::perona_default_embed;

#[command]
#[aliases("latency")]
pub async fn ping(context: &Context, message: &Message) -> CommandResult {
	let response_latency_start = Instant::now();
	let mut response = message.channel_id.send_message(&context.http, |message| {
		message.content("🏓 Calculando a latência... 🏓");
		return message;
	}).await.unwrap();
	let response_latency_end = response_latency_start.elapsed();
	let gateway_latency_start = Instant::now();
    context.http.get_gateway().await.unwrap();
    let gateway_latency_end = gateway_latency_start.elapsed();
	let embed_content = perona_default_embed(&context,
		String::from("👻 Informações sobre a latência da Perona 👻"),
		format!("🎈 Latência do getaway é de **_`{}ms`_**.\n🔥 Latência da api é de **_`{}ms`_**.",
			gateway_latency_end.as_millis(),
			response_latency_end.as_millis())
	).await;
	response.edit(&context.http, |edit| {
		edit
			.content("\u{0}") // * clear content of last message.
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
#[aliases("github")]
pub async fn source(context: &Context, message: &Message) -> CommandResult {
	let embed_content = perona_default_embed(&context,
		String::from("👻 Aqui está o meu código-fonte completo 👻"),
		String::from("📦 Meu código-fonte completo no Github : ***https://github.com/NukeZzZin/Perona.***")
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
