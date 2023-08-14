use serenity::{
	framework::standard::{
		macros::command,
		CommandResult,
		Args
	},
	model::prelude::channel::Message,
	prelude::Context
};
use rand::{
	thread_rng,
	Rng
};
use crate::utilities::functions::perona_default_embed;

// TODO: implement various commands for funny

#[command]
pub async fn dice(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
	let argument = arguments.single::<u32>().unwrap_or(7);
	let embed_content = perona_default_embed(&context,
		"👻 Resultado do dado jogado pela Perona 👻",
		format!("🎲 O valor do dado jogador por perona: **_`{}`_**.",
			thread_rng().gen_range(1..=argument) as u32) // * it's generate a random number about 1 and number passed from argument.
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
