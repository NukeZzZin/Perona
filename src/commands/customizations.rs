use serenity::{
	framework::standard::{
		macros::command,
		CommandResult,
		Args
	},
	model::prelude::channel::Message,
	prelude::Context
};
use mongodb::{
	bson::doc,
	options::{
		FindOneAndUpdateOptions,
		ReturnDocument
	}
};
use crate::utilities::functions::perona_default_embed;
use crate::utilities::structures::GuildsCollectionContainer;

#[command]
pub async fn prefix(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
	let argument = arguments.single::<String>().unwrap();
	let read = context.data.read().await;
	if let Some(collection) = read.get::<GuildsCollectionContainer>() {
		let old = collection.data.find_one_and_update(
			doc!{"_id":message.guild_id.unwrap().0.to_string()},
			doc!{"$set":{"guild_prefix":argument.clone()}},
			FindOneAndUpdateOptions::builder()
				.return_document(ReturnDocument::Before)
				.build()
		).await.unwrap();
		let embed_content = perona_default_embed(&context,
			String::from("👻 O prefixo da Perona foi alterado com sucesso 👻"),
			format!("✏️ O prefixo da Perona foi alterado de **_`{}`_** para **_`{}`_**.", old.unwrap().guild_prefix, argument)
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
