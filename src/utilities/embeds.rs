use serenity::{
	client::Context,
	model::channel::Message,
	builder::CreateEmbed
};

pub async fn perona_default_embed(context: &Context, message: &Message, title: String, description: String) -> CreateEmbed {
	let perona_image = context.http.get_current_user().await.ok().unwrap().avatar_url().unwrap();
	let mut callback = CreateEmbed::default();
	callback
		.author(|author| author.name(title))
		.description(description)
		.thumbnail("https://i.imgur.com/MYNjFgT.gif")
		.color(0xCC6699)
		.footer(|footer| footer.text("Senhorita Perona's").icon_url(&perona_image))
		.timestamp(message.timestamp.to_rfc3339());
	return callback.clone();
}
