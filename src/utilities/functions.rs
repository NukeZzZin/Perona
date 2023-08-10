use serenity::{
	builder::CreateEmbed,
	prelude::Context
};
use chrono::Utc;

pub async fn perona_default_embed(context: &Context, title: String, description: String) -> CreateEmbed {
	let perona_image = context.http.get_current_user().await.unwrap().avatar_url().unwrap();
	let mut callback = CreateEmbed::default();
	callback
		.author(|author| author.name(title))
		.description(description)
		.thumbnail("https://i.imgur.com/MYNjFgT.gif") // TODO: define it with default thumbnail image for embeds
		.color(0xCC_66_99u32)
		.footer(|footer| footer.text("Senhorita Perona's").icon_url(&perona_image))
		.timestamp(Utc::now());
	return callback;
}
