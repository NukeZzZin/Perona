use std::{
	env,
	time::Instant
};
use serenity::{
	framework::standard::{
		macros::command,
		CommandResult,
	},
	prelude::*,
	model::prelude::*
};
use crate::utilities::functions::perona_default_embed;

#[command]
pub async fn ping(context: &Context, message: &Message) -> CommandResult {
	let response_latency_start = Instant::now();
	let mut response = message.channel_id.send_message(&context.http, |content| {
		content.content("🏓 Calculando a latência... 🏓");
		content
	}).await.unwrap();
	let response_latency_end = response_latency_start.elapsed();
	let gateway_latency_start = Instant::now();
    context.http.get_gateway().await.unwrap();
    let gateway_latency_end = gateway_latency_start.elapsed();
	let embed_content = perona_default_embed(&context, &message,
		"👻 Informações sobre a latência da Perona 👻".to_string(),
		format!(">>> **🎈 Latência do getaway é de {}ms.**\n**🔥 Latência da api é de {}ms.**",
			gateway_latency_end.as_millis(),
			response_latency_end.as_millis())
	).await;
	response.edit(context, |edit| edit.embed(|embed| {
		embed.clone_from(&embed_content);
		embed
	})).await.unwrap();
	return CommandResult::Ok(());
}

#[command]
pub async fn invite(context: &Context, message: &Message) -> CommandResult {
	let embed_content = perona_default_embed(&context, &message,
		"👻 Link para convidar a Perona para seu servidor 👻".to_string(),
		format!(">>> **❤️ Me convide para seu servidor utilizando este link : {}.**",
			context.http.get_current_user().await.ok().unwrap().invite_url(&context.http, Permissions::all()).await.unwrap())
	).await;
	message.channel_id.send_message(&context, |content| {
		content.embed(|embed| {
			embed.clone_from(&embed_content);
			embed
		});
		content
	}).await.unwrap();
	return CommandResult::Ok(());
}

#[command]
pub async fn source(context: &Context, message: &Message) -> CommandResult {
	let embed_content = perona_default_embed(&context, &message,
		"👻 Aqui está o meu código-fonte completo 👻".to_string(),
		format!(">>> **📦 Meu código-fonte completo no Github : {}.**",
			env::var("GITHUB_REPO").expect("[-] Failed to find GITHUB_REPO in environment file."))
	).await;
	message.channel_id.send_message(&context.http, |content| {
		content.embed(|embed| {
			embed.clone_from(&embed_content);
			embed
		});
		content
	}).await.unwrap();
	return CommandResult::Ok(());
}
