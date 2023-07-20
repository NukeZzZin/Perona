use std::time::Instant;
use serenity::{
	framework::standard::{
		macros::command,
		// Args,
		CommandResult,
	},
	prelude::*,
	model::prelude::*
};

#[command]
pub async fn ping(context: &Context, message: &Message) -> CommandResult {
	let response_latency_start = Instant::now();
	let mut response = message.channel_id.send_message(&context.http, |content| {
		content.content('\0');
		content
	}).await.unwrap();
	let response_latency_end = Instant::now();
	let gateway_latency_start = Instant::now();
    context.http.get_gateway().await.unwrap();
    let gateway_latency_end = Instant::now();
	let perona_image = context.http.get_current_user().await.ok().unwrap().avatar_url().unwrap();
	response.edit(context, |edit| edit.embed(|embed| {
		embed
			.author(|author| author.icon_url(&perona_image).name("👻 Algumas informações sobre a latência da Senhorita Perona 👻"))
			.description(format!("**🎈 Latência do getaway é de {}ms**\n**🔥 Latência da api é de {}ms**",
				gateway_latency_end.duration_since(gateway_latency_start).as_millis(),
				response_latency_end.duration_since(response_latency_start).as_millis()))
			.image(&perona_image)
			.color(0x000000)
			.footer(|footer| footer.text("Senhorita Perona's"))
			.timestamp(message.timestamp.to_rfc3339());
		embed
	})).await.unwrap();
	return CommandResult::Ok(());
}

#[command]
pub async fn invite(context: &Context, message: &Message) -> CommandResult {
	let perona_image = context.http.get_current_user().await.ok().unwrap().avatar_url().unwrap();
	let server_invite = context.http.get_current_user().await.ok().unwrap().invite_url(&context.http, Permissions::all()).await.unwrap();
	message.channel_id.send_message(&context, |content| {
		content.embed(|embed| {
			embed
				.author(|author| author.icon_url(&perona_image).name("👻 Aqui está o convite para me convidar para seu servidor 👻"))
				.description(format!("**❤️ Me convide para seu servidor utilizando este link : {}**", server_invite))
				.image(&perona_image)
				.color(0x000000)
				.footer(|footer| footer.text("Senhorita Perona's"))
				.timestamp(message.timestamp.to_rfc3339())
		});
		content
	}).await.unwrap();
	return CommandResult::Ok(());
}
