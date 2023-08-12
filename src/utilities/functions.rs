use serenity::{
	builder::CreateEmbed,
	prelude::Context,
};
use chrono::Utc;


#[allow(dead_code)]
#[derive(PartialEq, PartialOrd)]
pub enum PeronaLoggerStatus {
    Info = 1 << 0,
    Warning = 1 << 1,
    Error = 1 << 2,
	Debug = 1 << 3,
	Fatal = 1 << 4
}

#[macro_export]
macro_rules! perona_println {
	($status:expr, $($arg:tt)*) => {
		{
			let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
			if $status <= PeronaLoggerStatus::Info // * it's manually sets minimum log value.
			{
				match $status
				{
					PeronaLoggerStatus::Info => {
						println!("[\x1b[1;32mINFO\x1b[0m] (\x1b[1;90m{}\x1b[0m) - {}.", timestamp, format_args!($($arg)*));
					},
					PeronaLoggerStatus::Warning => {
						println!("[\x1b[1;33mWARNING\x1b[0m] (\x1b[1;90m{}\x1b[0m) - {}.", timestamp, format_args!($($arg)*));
					},
					PeronaLoggerStatus::Error => {
						eprintln!("[\x1b[1;91mERROR\x1b[0m] (\x1b[1;90m{}\x1b[0m) - {}.", timestamp, format_args!($($arg)*));
					},
					PeronaLoggerStatus::Debug => {
						println!("[\x1b[1;36mDEBUG\x1b[0m] (\x1b[1;90m{}\x1b[0m) - [{}:{}] {}.", timestamp, file!(), line!(), format_args!($($arg)*));
					},
					PeronaLoggerStatus::Fatal => {
						println!("[\x1b[1;91mFATAL\x1b[0m] (\x1b[1;90m{}\x1b[0m) - {}.", timestamp, format_args!($($arg)*));
						std::process::exit(0x5442 as i32); // TODO: define perona return status codes
					}
				}
			}
		}
	};
}

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
