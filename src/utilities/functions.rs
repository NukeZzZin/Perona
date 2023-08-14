use serenity::{
	builder::CreateEmbed,
	prelude::Context
};
use chrono::Utc;


#[allow(dead_code)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum PeronaLoggerStatus {
	// Trace = (0x1 << 0x0),
	Debug = (0x1 << 0x1),
    Info = (0x1 << 0x2),
    Warning = (0x1 << 0x3),
    Error = (0x1 << 0x4),
	Fatal = (0x1 << 0x5)
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
					// PeronaLoggerStatus::Trace => {
					// 	println!("[\x1b[1;35mTRACE\x1b[0m] (\x1b[1;90m{}\x1b[0m) - [{}:{}] {}.", timestamp, file!(), line!(), format_args!($($arg)*));
					// },
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

pub async fn perona_default_embed<T, U>(context: &Context, title: T, description: U) -> CreateEmbed
where
	T: ToString,
	U: ToString
{
	let perona_image = context.http.get_current_user().await.unwrap().avatar_url().unwrap();
	let mut callback = CreateEmbed::default();
	callback
		.author(|author| author.name(title.to_string()))
		.description(description.to_string())
		.thumbnail("https://i.imgur.com/MYNjFgT.gif") // TODO: define it with default thumbnail image for embeds
		.color(0xCC_66_99u32)
		.footer(|footer| footer.text("Senhorita Perona's").icon_url(&perona_image))
		.timestamp(Utc::now());
	return callback;
}
